# -*- coding: utf-8 -*-

import requests_mock
import socket

from dike import get_remote_ip, get_local_ip


def test_get_remote_ip():
    with requests_mock.Mocker() as mock:
        mock.get('https://api.ipify.org', text='192.168.0.1')
        assert get_remote_ip(), '192.168.0.1'


def test_get_local_ip():
    assert get_local_ip(), socket.gethostbyname(socket.gethostname())
