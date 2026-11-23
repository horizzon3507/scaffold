FROM rust:1.100-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/{{name}} /usr/local/bin/{{name}}
ENTRYPOINT ["{{name}}"]
