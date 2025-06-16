FROM rust:1.87.0-alpine3.22 AS builder

RUN apk update
RUN apk add libc-dev openssl-dev build-base musl-dev pkgconfig openssl-libs-static

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.22
COPY --from=builder /app/target/release/ductaper /usr/local/bin/ductaper

RUN adduser -D user
RUN mkdir -p /home/user/.local
COPY data/system_prompt.txt /home/user/.local/system_prompt.txt
RUN chown -R user:user /home/user
COPY healthcheck.sh /
RUN chown root:user /healthcheck.sh && chmod 550 /healthcheck.sh
USER user
CMD ["ductaper"]
