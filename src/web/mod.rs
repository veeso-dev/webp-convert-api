use std::net::{SocketAddr, TcpListener};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App as ActixApp, HttpRequest, HttpServer};

mod health_check;
mod webp_convert;

const MAX_SIZE_BYTES_20MB: usize = 20 * 1024 * 1024;

pub struct WebServer {
    server: Server,
}

struct WebserverData {
    apikey: String,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(apikey: String, listener_addr: SocketAddr) -> anyhow::Result<Self> {
        info!("webserver initialized");
        info!("listener address: {listener_addr}");

        let listener = TcpListener::bind(listener_addr)?;

        let server = {
            HttpServer::new(move || {
                let apikey = apikey.clone();
                let web_data = Data::new(WebserverData { apikey });
                ActixApp::new()
                    .service(health_check::check_action)
                    .service(webp_convert::webp_convert)
                    .service(webp_convert::webp_resize)
                    .app_data(actix_web::web::PayloadConfig::new(MAX_SIZE_BYTES_20MB)) // set max payload size to 20MB
                    .app_data(web_data)
                    .wrap(Logger::default())
            })
            .listen(listener)?
            .run()
        };

        info!("web server initialized");
        Ok(Self { server })
    }

    /// Run web server
    pub async fn run(self) -> anyhow::Result<()> {
        info!("running web server");
        self.server.await?;
        info!("web server stopped");
        Ok(())
    }

    /// Check API key
    fn check_apikey(apikey: &str, request: &HttpRequest) -> bool {
        if let Some(header) = request.headers().get("x-api-key") {
            if let Ok(value) = header.to_str() {
                return value == apikey;
            }
        }
        false
    }
}
