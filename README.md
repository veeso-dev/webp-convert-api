# Webp-convert-api

[![license-mit](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/license/mit/)
[![build-test](https://github.com/veeso-dev/webp-convert-api/actions/workflows/build-test.yml/badge.svg)](https://github.com/veeso-dev/webp-convert-api/actions/workflows/build-test.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

---

- [Webp-convert-api](#webp-convert-api)
  - [About webp-convert-api](#about-webp-convert-api)
  - [Get started](#get-started)
    - [Setup env](#setup-env)
    - [Run with Cargo make](#run-with-cargo-make)
  - [webp-convert-api API](#webp-convert-api-api)
    - [Check](#check)
    - [Convert](#convert)
    - [Resize](#resize)
  - [Contributing and issues](#contributing-and-issues)
  - [Changelog](#changelog)
  - [License](#license)

---

## About webp-convert-api

webp-convert-api is a Rust web service which comes integrated with ClamAV. The service provides an API endpoint to scan files with ClamAV.

---

## Get started

### Setup env

```sh
cp .env.test .env
vim .env
```

```env
APIKEY={your_api_key}
WEB_PORT=3001
```

### Run with Cargo make

```sh
cargo make -p production run
```

## webp-convert-api API

### Check

Check web service status:

```txt
GET /check
```

Response: Empty (200)

### Convert

```txt
POST /convert
```

Body: image binary

Response: image/webp with webp image data

### Resize

```txt
POST /resize/:width/:height
```

Body: image binary

Response: image/webp with webp image data

---

## Contributing and issues

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve pavao, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog

View webp-convert-api's changelog [HERE](CHANGELOG.md)

---

## License

webp-convert-api is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
