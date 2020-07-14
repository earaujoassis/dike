# Dike client [![Build Status](https://travis-ci.com/earaujoassis/dike-client.svg?branch=master)](https://travis-ci.com/earaujoassis/dike-client) [![codecov](https://codecov.io/gh/earaujoassis/dike-client/branch/master/graph/badge.svg)](https://codecov.io/gh/earaujoassis/dike-client)

> Dike client is a really tiny system to provide a dynamic DNS (DDNS) service

Dike client periodically connects to a HTTP server which servers as a Remote Backend for the
PowerDNS DNS service provider. It `POST` messages about the current local and remote network
IP for a device. Alongside [Dike server](https://github.com/earaujoassis/dike-server), it
works just like DDNS services like Dyn or No-Ip. It is intendend for private, IoT-oriented,
residential use only (security measures are taken).

## Setup & Running

First you need to create and to activate the `venv` and install the requirements. Please make sure
to use Python 3. You may use [`asdf`](https://github.com/asdf-vm/asdf) to manage Python versions; a
`.tool-versions` file is already available.

```sh
$ python3 -m venv venv
$ source venv/bin/activate
$ pip install -r requirements.txt
```

## Testing

Once all setup is done, testing can be done through the following command:

```sh
$ pytest
```

## Issues

Please take a look at [/issues](https://github.com/earaujoassis/dike-client/issues)

## License

[MIT License](http://earaujoassis.mit-license.org/) &copy; Ewerton Carlos Assis
