#!/usr/bin/env python
# -*- coding: utf-8 -*-
#

import argparse
import sys
import os
import re

if sys.version_info[0] < 3:
    from urlparse import urlparse
else:
    from urllib.parse import urlparse


COMMAND_TO_FUNCTION = {
    "user": "username",
    "passwd": "password",
    "host": "hostname",
    "port": "port",
}


parser = argparse.ArgumentParser(description="urlparser tool")
subparsers = parser.add_subparsers(dest="parent")
subparsers.add_parser("user", help="extract username")
subparsers.add_parser("passwd", help="extract password")
subparsers.add_parser("host", help="extract hostname")
subparsers.add_parser("port", help="extract port")


def format_value(value):
    if value is None:
        return ""
    return value


def username(url_parsed):
    sys.stdout.write("{0}".format(format_value(url_parsed.username)))


def password(url_parsed):
    sys.stdout.write("{0}".format(format_value(url_parsed.password)))


def hostname(url_parsed):
    sys.stdout.write(url_parsed.hostname)


def port(url_parsed):
    sys.stdout.write("{0}".format(format_value(url_parsed.port)))


def main(argv):
    namespace = parser.parse_args(argv[1:])
    parent = getattr(namespace, "parent")
    function_parent = globals()[COMMAND_TO_FUNCTION.get(parent)]
    url = "".join(sys.stdin.readlines()).strip()
    url_parsed = urlparse(url)
    function_parent(url_parsed)


if __name__ == "__main__":
    main(sys.argv)
