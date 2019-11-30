FROM rust:latest

WORKDIR /usr/src/lucid
COPY . .

RUN cargo build --release
RUN cp target/release/lucid-server /usr/bin/

EXPOSE 7021

CMD ["lucid-server", "--config", "/etc/lucid/lucid.yml"]
