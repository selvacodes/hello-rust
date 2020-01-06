FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/hello-rust

COPY . .

RUN cargo install --path . 

FROM debian:buster-slim

COPY --from=builder /usr/src/hello-rust/target/release /bin/hello-rust 

WORKDIR /root

RUN chmod 777 /bin/hello-rust

CMD ROCKET_PORT=8000 /bin/hello-rust

