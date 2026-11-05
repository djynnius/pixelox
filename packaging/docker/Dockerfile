# pixelox in a container:
#   docker build -f packaging/docker/Dockerfile -t pixelox .
#   docker run --rm -v "$PWD:/work" -w /work pixelox info photo.jpg --json
FROM rust:1.97-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p pixelox

FROM debian:bookworm-slim
COPY --from=build /src/target/release/pixelox /usr/local/bin/pixelox
ENV PIXELOX_HOME=/data PIXELOX_SYSTEM_FONTS=0
VOLUME ["/data"]
WORKDIR /work
ENTRYPOINT ["pixelox"]
CMD ["--help"]
