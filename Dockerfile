FROM rust:latest as builder

RUN USER=root cargo new --bin urusai
WORKDIR ./urusai

RUN touch ./src/lib.rs

RUN apt-get update \
    && apt-get install -y curl \
    && curl -sL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./package.json ./package.json
COPY ./package-lock.json ./package-lock.json

RUN npm install

ADD . ./

RUN rm ./target/release/deps/urusai*
RUN cargo build --release

FROM nginx:latest as urusai_nginx

COPY --from=builder /urusai/public /opt/urus.ai/public

ADD ./nginx/nginx.conf /etc/nginx/nginx.conf

FROM christophwurst/diesel-cli as urusai_migrations

COPY --from=builder /urusai/migrations /migrations

CMD ["diesel", "database", "setup", "--migration-dir", "/migrations"]

FROM debian:buster-slim as urusai

ARG APP=/usr/src/app

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER

COPY --from=builder /urusai/target/release/urusai ${APP}/urusai

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./urusai"]
