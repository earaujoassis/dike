FROM rust:1.30.0

ARG ROCKET_ENV=production
ARG ENV=production

ENV ROCKET_ENV=$ROCKET_ENV
ENV ENV=$ENV

RUN apt-get update
RUN apt-get install -y mysql-client

WORKDIR /usr/src/dike
COPY . .

RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install --path .

EXPOSE 80
CMD ./scripts/container.sh
