FROM messense/rust-musl-cross:x86_64-musl AS builder
RUN rustup update nightly && rustup target add --toolchain nightly x86_64-unknown-linux-musl
COPY . /home/rust/rockhttp/
WORKDIR /home/rust/rockhttp
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo +nightly build -Z build-std=std,panic_abort --target x86_64-unknown-linux-musl --release
RUN musl-strip /home/rust/rockhttp/target/x86_64-unknown-linux-musl/release/rockhttp
RUN echo $(ldd /home/rust/rockhttp/target/x86_64-unknown-linux-musl/release/rockhttp)
RUN mkdir -p /home/rust/rockhttp/files

FROM busybox:stable-musl
COPY --from=builder /home/rust/rockhttp/target/x86_64-unknown-linux-musl/release/rockhttp /rockhttp/rockhttp
COPY --from=builder /home/rust/rockhttp/assets/ /rockhttp/assets/
COPY --from=builder /home/rust/rockhttp/pages/ /rockhttp/pages/
COPY --from=builder /home/rust/rockhttp/Rocket.toml /rockhttp/Rocket.toml
COPY --from=builder /home/rust/rockhttp/files/ /rockhttp/files/ 
WORKDIR /rockhttp
CMD ["./rockhttp"]
EXPOSE 8000