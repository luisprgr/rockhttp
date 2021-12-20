# Rockhttp
> A http file server made in the Rust web framework Rocket with the template engine Tera

## Build

1. Install the Rust toolchain 
2. build with ``cargo build --release``
3. Run the application with ``cargo run --release`` or with ``./rockhttp`` in the *target/release* folder 

## Installation with docker 

You can use the Dockerized version with:

```shell
Docker run -P 8000:8000 luisprgr/rockhttp:latest
```

Or creating a docker-compose.yml like the following example:

```shell
version: "3.3"
services:
  rockhttp:
    image: luisprgr/rockhttp
    container_name: rockhttp
    ports:
      - 8000:8000
```