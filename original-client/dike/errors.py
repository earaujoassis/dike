# -*- coding: utf-8 -*-

class Error(Exception):
    pass


class RemoteServerError(Error):
    def __init__(self, action, response, message):
        self.action = action
        self.response = response
        self.message = message
