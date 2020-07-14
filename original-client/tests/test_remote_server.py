# -*- coding: utf-8 -*-

import os
import pytest
import requests_mock

import dike.errors
from dike import revise_domain_entry



def revise_domain_entry_callback(request, context):
    context.status_code = 200
    json = request.json()
    return {
        "id": json.get("id", 10000),
        "subdomain": json.get("subdomain"), # only for testing
        "remote_ip": json.get("remote_ip"), # only for testing
        "sequence_number": 30
    }


def test_revise_domain_entry_for_new_entity():
    os.environ["DIKE_URI"] = "https://dike-server.com"
    with requests_mock.Mocker() as mock:
        mock.post('https://dike-server.com/records/entry', status_code=200, json=revise_domain_entry_callback)
        returned_payload = revise_domain_entry(subdomain="testing.mydomain.com", remote_ip="192.168.0.1")
        assert returned_payload.get("id"), 10000
        assert returned_payload.get("subdomain"), "testing.mydomain.com"
        assert returned_payload.get("remote_ip"), "192.168.0.1"
        assert returned_payload.get("sequence_number"), 30


def test_revise_domain_entry_for_existent_entity():
    os.environ["DIKE_URI"] = "https://dike-server.com"
    with requests_mock.Mocker() as mock:
        mock.post('https://dike-server.com/records/entry', status_code=200, json=revise_domain_entry_callback)
        returned_payload = revise_domain_entry(id=12345, subdomain="testing.mydomain.com", remote_ip="192.168.0.1")
        assert returned_payload.get("id"), 12345
        assert returned_payload.get("subdomain"), "testing.mydomain.com"
        assert returned_payload.get("remote_ip"), "192.168.0.1"
        assert returned_payload.get("sequence_number"), 30


def test_revise_domain_entry_for_error():
    os.environ["DIKE_URI"] = "https://dike-server.com"
    with requests_mock.Mocker() as mock:
        mock.post('https://dike-server.com/records/entry', status_code=400, text="Missing data")
        with pytest.raises(dike.errors.RemoteServerError) as exception_raised:
            revise_domain_entry()
        assert exception_raised.value.response.status_code, 400
        assert exception_raised.value.action, "revise_domain_entry"
        assert exception_raised.value.message, "Missing data"
