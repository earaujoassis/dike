# Dike server

> Dike server is a really tiny dynamic DNS (DDNS) server

Dike server is based on the PowerDNS Remote Backend: it makes available a backend HTTP server
to provide queries answers to the DNS server; and a HTTP server to attend dynamic DNS (DDNS) clients.
It uses the [Actix web framework](https://actix.rs/) + [Diesel ORM](http://diesel.rs/) for
the Rust language.

## Setup & Running

Actix requires the nightly builds for Rust. A `.env` file must be created, according to the `.env.sample`.
After installing Rust and setting up Cargo, the following commands will (1) install the `diesel_cli` binary;
(2) migrate migrations; and (3) compile the application and run it:

```sh
$ cargo install diesel_cli --no-default-features --features mysql
$ diesel migration run --database-url=?
$ cargo run
```

The HTTP server's address will be printed in the console.

Another possibility is using `docker-compose` to start all services (including database):

```sh
$ docker-compose build
$ docker-compose up
```

## Issues

Please take a look at [/issues](https://github.com/earaujoassis/dike-server/issues)

## License

[MIT License](http://earaujoassis.mit-license.org/) &copy; Ewerton Carlos Assis
