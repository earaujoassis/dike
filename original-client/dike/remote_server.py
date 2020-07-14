# -*- coding: utf-8 -*-

import requests
import os

from .settings import *
from .errors import RemoteServerError


def revise_domain_entry(**kwargs):
    payload = dict((k,v) for k, v in kwargs.items() if k in ["id", "subdomain", "remote_ip"])
    response = requests.post("{0}/records/entry".format(os.getenv("DIKE_URI")),
                             json=payload)
    if response.status_code != 200:
        raise RemoteServerError(action="revise_domain_entry",
                                response=response,
                                message=response.text)
    return response.json()
