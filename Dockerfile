FROM rustlang/rust:nightly
MAINTAINER Ewerton Carlos Assis <earaujoassis@gmail.com>

ARG ROCKET_ENV=production
ARG ENV=production

ENV ROCKET_ENV=$ROCKET_ENV
ENV ENV=$ENV

WORKDIR /usr/src/knock_knock
COPY . .
RUN cargo install --path .

EXPOSE 5000
CMD [ "cargo", "run" ]
