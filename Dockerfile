FROM debian:bullseye-slim
WORKDIR /app
ADD target/debug/actix-test .
CMD ["/app/actix-test"]
