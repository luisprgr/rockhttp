FROM rust:slim-bullseye AS builder
COPY . /rockhttp/
WORKDIR rockhttp
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /rockhttp/target/release/rockhttp /rockhttp/rockhttp
COPY --from=builder /rockhttp/assets/ /rockhttp/assets/
COPY --from=builder /rockhttp/pages/ /rockhttp/pages/
COPY --from=builder /rockhttp/Rocket.toml /rockhttp/Rocket.toml
WORKDIR /rockhttp
CMD ["./rockhttp"]
EXPOSE 8000