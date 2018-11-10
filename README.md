# Dike server [![Build Status](https://travis-ci.com/earaujoassis/dike-server.svg?branch=master)](https://travis-ci.com/earaujoassis/dike-server)

> Dike server is a really tiny dynamic DNS (DDNS) server

Dike server is based on the PowerDNS Remote Backend: it makes available a backend HTTP server
to provide queries answers to the DNS server; and a HTTP server to attend dynamic DNS (DDNS) clients.
It uses the [Actix web framework](https://actix.rs/) + [Diesel ORM](http://diesel.rs/) for
the Rust language. Notice: the Actix web framework requires the nightly builds for Rust.

## Setup & Running

The current application uses MariaDB as the backend database. For development purposes, it is possible
to start the `dike-mariadb` container if Docker + docker-compose is available through the following
command: `$ docker-compose up -d --build dike-mariadb`. A `.env` file must be created, according to the
`.env.sample`. After installing Rust and setting up Cargo, the following command will (1) install the
`diesel_cli` binary (if that is not available); (2) migrate any new *migration*; and (3) compile the
application and run it:

```sh
$ scripts/container.sh
```

The HTTP server's address will be printed in the console.

Another possibility is using `docker-compose` to start all services (including database):

```sh
$ docker-compose up --build
```

## Testing & Linting

For testing (plus code coverage) and linting, you could run:

```sh
$ scripts/test.sh
```

## Issues

Please take a look at [/issues](https://github.com/earaujoassis/dike-server/issues)

## License

[MIT License](http://earaujoassis.mit-license.org/) &copy; Ewerton Carlos Assis
