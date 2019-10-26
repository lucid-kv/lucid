#MAINTAINER clint.mourlevat@gmail.com

# Build Lucid
FROM rust:1.38 as build
COPY ./ ./
RUN cargo build --release
RUN mkdir -p /output
RUN cp target/release/lucid /output

# Run Lucid
FROM alpine:latest
COPY --from=build /output/lucid /
WORKDIR /
CMD ["lucid", "init"]
CMD ["lucid", "server"]
