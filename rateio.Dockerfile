FROM rust:1.85-alpine3.20 AS builder

WORKDIR /app
COPY . .

WORKDIR /app/rateio-api
RUN cargo build --release

FROM scratch AS runner
WORKDIR /app
COPY --from=builder --chmod=700 /app/rateio-api/target/release/rateio-api /app/rateio-api
CMD ["/app/rateio-api"]
