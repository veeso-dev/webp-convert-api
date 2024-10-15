use std::io::Cursor;

use actix_web::web::{self, Bytes, Data};
use actix_web::{post, HttpRequest, HttpResponse};
use image::ImageFormat;

use super::{WebServer, WebserverData};

#[post("/convert")]
pub async fn webp_convert(
    data: Data<WebserverData>,
    request: HttpRequest,
    image_data: Bytes,
) -> HttpResponse {
    // check apikey
    if !WebServer::check_apikey(&data.apikey, &request) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let webp_data = match convert_image_to_webp(image_data, None) {
        Ok(webp_data) => webp_data,
        Err(response) => return response,
    };

    HttpResponse::Ok()
        .content_type("image/webp")
        .body(webp_data)
}

#[post("/resize/{width}/{height}")]
pub async fn webp_resize(
    data: Data<WebserverData>,
    path: web::Path<(u32, u32)>,
    request: HttpRequest,
    image_data: Bytes,
) -> HttpResponse {
    // check apikey
    if !WebServer::check_apikey(&data.apikey, &request) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let (width, height) = path.into_inner();

    let webp_image = match convert_image_to_webp(image_data, Some((width, height))) {
        Ok(webp_image) => webp_image,
        Err(response) => return response,
    };

    HttpResponse::Ok()
        .content_type("image/webp")
        .body(webp_image)
}

fn convert_image_to_webp(
    bytes: Bytes,
    resize: Option<(u32, u32)>,
) -> Result<Vec<u8>, HttpResponse> {
    let Ok(mut image) = image::load_from_memory(&bytes) else {
        return Err(HttpResponse::BadRequest().body("provided data is not an image"));
    };

    if let Some((width, height)) = resize {
        image = image.resize(width, height, image::imageops::FilterType::Lanczos3);
    }

    let mut webp_data = Vec::new();
    if image
        .write_to(&mut Cursor::new(&mut webp_data), ImageFormat::WebP)
        .is_err()
    {
        return Err(HttpResponse::InternalServerError().body("Failed to convert image to WebP"));
    }

    Ok(webp_data)
}

#[cfg(test)]
mod test {

    use actix_web::body::MessageBody;
    use actix_web::dev::Service;
    use actix_web::http::{Method, StatusCode};
    use actix_web::{test, App};

    use super::*;
    use crate::web::MAX_SIZE_BYTES_20MB;

    #[actix_rt::test]
    async fn test_should_convert_image_to_webp() {
        crate::test::log_init();
        let data = Data::new(WebserverData {
            apikey: "abcdef".to_string(),
        });

        let image_data = include_bytes!("../../test/test-img.png").to_vec();
        assert!(image_data.len() > 0);

        let app = test::init_service(
            App::new()
                .app_data(data)
                .app_data(actix_web::web::PayloadConfig::new(MAX_SIZE_BYTES_20MB)) // set max payload size to 20MB
                .service(webp_convert),
        )
        .await;
        let request = test::TestRequest::with_uri("/convert")
            .method(Method::POST)
            .insert_header(("content-type", "image/png"))
            .insert_header(("x-api-key", "abcdef"))
            .set_payload(image_data)
            .to_request();

        let res = app.call(request).await.unwrap();
        println!("{:?}", res.status());
        assert!(res.status().is_success());

        // get webp
        let webp_data = res
            .map_body(|_, body| body.try_into_bytes().expect("failed to get data").to_vec())
            .response()
            .body()
            .clone();

        assert!(webp_data.len() > 0);
        assert!(image::load_from_memory(&webp_data).is_ok());
    }

    #[actix_rt::test]
    async fn test_should_resize_image() {
        crate::test::log_init();
        let data = Data::new(WebserverData {
            apikey: "abcdef".to_string(),
        });

        let image_data = include_bytes!("../../test/test-img.png").to_vec();
        assert!(image_data.len() > 0);

        let app = test::init_service(
            App::new()
                .app_data(data)
                .app_data(actix_web::web::PayloadConfig::new(MAX_SIZE_BYTES_20MB)) // set max payload size to 20MB
                .service(webp_resize),
        )
        .await;
        let request = test::TestRequest::with_uri("/resize/100/100")
            .method(Method::POST)
            .insert_header(("content-type", "image/png"))
            .insert_header(("x-api-key", "abcdef"))
            .set_payload(image_data)
            .to_request();

        let res = app.call(request).await.unwrap();
        println!("{:?}", res.status());
        assert!(res.status().is_success());

        // get webp
        let webp_data = res
            .map_body(|_, body| body.try_into_bytes().expect("failed to get data").to_vec())
            .response()
            .body()
            .clone();

        assert!(webp_data.len() > 0);
        let image = image::load_from_memory(&webp_data).expect("failed to load image");

        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }

    #[actix_rt::test]
    async fn test_should_reject_non_image() {
        crate::test::log_init();
        let data = Data::new(WebserverData {
            apikey: "abcdef".to_string(),
        });

        let image_data = include_bytes!("../../test/bad-img.jpeg").to_vec();
        assert!(image_data.len() > 0);

        let app = test::init_service(App::new().app_data(data).service(webp_convert)).await;
        let request = test::TestRequest::with_uri("/convert")
            .method(Method::POST)
            .insert_header(("content-type", "image/png"))
            .insert_header(("x-api-key", "abcdef"))
            .set_payload(image_data)
            .to_request();

        let res = app.call(request).await.unwrap();
        println!("{:?}", res.status());
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_should_reject_missing_apikey() {
        crate::test::log_init();
        let data = Data::new(WebserverData {
            apikey: "abcdef".to_string(),
        });

        let image_data = include_bytes!("../../test/bad-img.jpeg").to_vec();
        assert!(image_data.len() > 0);

        let app = test::init_service(App::new().app_data(data).service(webp_convert)).await;
        let request = test::TestRequest::with_uri("/convert")
            .method(Method::POST)
            .insert_header(("content-type", "image/png"))
            .set_payload(image_data)
            .to_request();

        let res = app.call(request).await.unwrap();
        println!("{:?}", res.status());
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
