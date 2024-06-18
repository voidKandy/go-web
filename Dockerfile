# FROM golang:1.21.3-alpine AS builder
# RUN mkdir /build
# ADD src/ /build/src/
# ADD public/ /build/public/
# ADD private/ /build/private/
# ADD migrations/ /build/migrations/
# ADD templates/ /build/template/
# WORKDIR /build
# RUN go build
#
# FROM alpine
# RUN adduser -S -D -H -h /app appuser
# USER appuser
# COPY --from=builder /build/voidkandy-dot-space /app/
# COPY public/ /app/public
# WORKDIR /app
# CMD ["./voidkandy-dot-space"]
FROM rust:latest as builder

RUN USER=root cargo new --bin voidkandy-dot-space
WORKDIR ./voidkandy-dot-space
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
 
ADD . ./


RUN rm ./target/release/voidkandy-dot-space
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
COPY --from=builder /voidkandy-dot-space/target/release/voidkandy-dot-space ${APP}/voidkandy-dot-space

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

# CMD ["docker-compose up -d; ./voidkandy-dot-space"]
CMD ["./voidkandy-dot-space"]
