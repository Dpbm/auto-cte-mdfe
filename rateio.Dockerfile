FROM rust:1.85-alpine3.20 AS builder

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev

WORKDIR /app/rateio-api
RUN cargo build --release

FROM alpine:3.23.0 AS runner

RUN apk add --no-cache musl-dev curl ca-certificates

ARG BUILD_HOST=0.0.0.0
ARG BUILD_PORT=4000
ARG BUILD_DATA_PATH=/data/

ENV HOST=$BUILD_HOST
ENV PORT=$BUILD_PORT
ENV DATA_PATH=$BUILD_DATA_PATH

WORKDIR /app
COPY --from=builder --chmod=700 /app/rateio-api/target/release/rateio-api rateio-api

EXPOSE $PORT

HEALTHCHECK --interval=10s \
  CMD curl -I -sf -o /dev/null http://$HOST:$PORT/health || exit 1

CMD ["/app/rateio-api"]
