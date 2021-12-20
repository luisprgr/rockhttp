# Rockhttp
> A http file server made in the Rust web framework Rocket with the template engine Tera

**[⚠] This is a experimental branch created to try to reduce the size of the docker container to the minimum possible, using rust nightly, a musl compiler and busybox as the base image [⚠]**

## Installation with docker 

```shell
Docker run -P 8000:8000 luisprgr/rockhttp:alpha-musl-busybox
```

Or creating a docker-compose.yml like the following example:

```shell
version: "3.3"
services:
  rockhttp:
    image: luisprgr/rockhttp:alpha-musl-busybox
    container_name: rockhttp
    ports:
      - 8000:8000
```

## License

Licensed under the [MIT License](LICENSE) 