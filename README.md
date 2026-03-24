# Network Security T - Assignment 1: App

## Project Information

| Name                        | NRP        | Group    |
|-----------------------------|------------|----------|
| Mohamad Valdi Ananda Tauhid | 5025221238 | opensmh  |

### Deployments

| Environment |  Link                                        |  Description    |
|:-----------:|----------------------------------------------|-----------------|
|  Keycloak   | https://keycloak.opensmh.shinyshoe.net       | The app         |
|  Keycloak   | https://auth.keycloak.opensmh.shinyshoe.net  | The OIDC server |
|  Ory Hydra  | https://ory-hydra.opensmh.shinyshoe.net      | The app         |
|  Ory Hydra  | https://auth.ory-hydra.opensmh.shinyshoe.net | The OIDC server |

### Docker Packages
|Application|Package|
|-|-|
|[app](https://github.com/valdi-at/network-security-t-assignment-1-app)|[ghcr.io/valdi-at/network-security-t-assignment-1-app](https://ghcr.io/valdi-at/network-security-t-assignment-1-app)|


### Source Code

|Submodule|Description|
|-|-|
|[**app**](https://github.com/valdi-at/network-security-t-assignment-1-app)|Source code for the app that will use the OIDC server|
|[keycloak](https://github.com/valdi-at/network-security-t-assignment-1-keycloak)|The stack containing the information about the environment that uses Keycloak as its OIDC server|
|[ory-hydra](https://github.com/valdi-at/network-security-t-assignment-1-ory-hydra)|The stack containing the information about the environment that uses Ory Hydra as its OIDC server|

*bold indicate the current submodule you are in

## Repository Information
This repository contains the application that will be used to integrate to the OIDC that will tested. The app will be written in the Rust Programming Language.

## Rust
This section is an instruction to run, test, dev, and compile the rust project. You probably should not need to read this unless you know what you're doing, its better to just run the docker project.

### Running the project

```bash
cargo leptos watch
```

### Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

### Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/release and your site package in target/site

### Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.


### Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
{{project-name}}
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="{{project-name}}"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Docker

### Building
``` bash
docker build -t THE_TAG .
```
### Environment Variable

#### Example
``` bash
LEPTOS_OUTPUT_NAME="network-security-t-assignment-1-app"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```