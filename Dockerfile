FROM rust:latest as builder

RUN USER=root cargo new --bin voidkandy-dot-space
WORKDIR ./voidkandy-dot-space
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
 
ADD . ./


RUN rm ./target/release/voidkandy-dot-space
RUN cargo build --release


FROM linuxcontainers/debian-slim:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libssl3\
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /voidkandy-dot-space/target/release/voidkandy-dot-space ${APP}/voidkandy-dot-space

RUN chown -R $APP_USER:$APP_USER ${APP}

WORKDIR ${APP}

USER root

ADD public ./public
ADD private ./private
ADD migrations ./migrations
ADD templates ./templates

RUN chown -R $APP_USER:$APP_USER ./public
RUN chmod -R 755 ./public  

USER $APP_USER

CMD ["./voidkandy-dot-space"]
