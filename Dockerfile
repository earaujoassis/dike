FROM rustlang/rust:nightly
MAINTAINER Ewerton Carlos Assis <earaujoassis@gmail.com>

ARG ROCKET_ENV=production
ARG ENV=production

ENV ROCKET_ENV=$ROCKET_ENV
ENV ENV=$ENV

RUN apt-get update
RUN apt-get install -y mysql-client

WORKDIR /usr/src/knock_knock
COPY . .

RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install --path .

EXPOSE 8000
CMD ./scripts/wait-for-datastore.sh datastore && ./scripts/start-knocking.sh
