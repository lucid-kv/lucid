FROM rust:latest

WORKDIR /usr/src/lucid
COPY . .

RUN cargo build --release
RUN cp target/release/lucid /usr/bin/

EXPOSE 7020
EXPOSE 7021

CMD ["lucid", "--config", "/etc/lucid/lucid.yml", "server"]
