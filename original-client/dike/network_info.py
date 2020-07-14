# -*- coding: utf-8 -*-

import requests
import socket


def get_remote_ip():
    return requests.get('https://api.ipify.org').text


def get_local_ip():
    return socket.gethostbyname(socket.gethostname())
