FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87-alpine3.22 AS chef

FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87-alpine3.22 AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87-alpine3.22 AS builder
RUN apk update
RUN apk add libc-dev openssl-dev build-base musl-dev pkgconfig openssl-libs-static
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

# Now build actual source
COPY . .
RUN cargo build --release

FROM alpine:3.22
COPY --from=builder /app/target/release/ductaper /usr/local/bin/ductaper
RUN mkdir -p /.local
COPY data/system_prompt.txt /.local/system_prompt.txt  
CMD ["ductaper"]
