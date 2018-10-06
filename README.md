# Knock-knock

> Knock-knock is a really tiny dynamic DNS (DDNS) server

Knock-knock is based on the PowerDNS Remote Backend: it makes available a backend HTTP server
to provide queries answers to the DNS server; and a HTTP server to attend dynamic DNS (DDNS) clients.
It uses the Rocket web framework + Diesel ORM for the Rust language.

## Setup & Running

Rocket requires the nightly builds for Cargo. You should check Rocket's documentation in order to
setup its current environment. You also must create a `.env` and `Rocket.toml` files, according to
each `.sample` for them (`.env.sample` and `Rocket.toml.sample` files, respectively). After installing
Rust and setting up Cargo, you may run:

```sh
$ cargo install diesel_cli --no-default-features --features mysql
$ diesel migration run
$ cargo run
```

It will compile the binary and run it. The HTTP server's address will be printed in the console.

Another possibility is using `docker-compose` to start all services (including database):

```sh
$ docker-compose up --build
```

## Issues

Please take a look at [/issues](https://github.com/earaujoassis/knock-knock/issues)

## License

[MIT License](http://earaujoassis.mit-license.org/) &copy; Ewerton Carlos Assis
