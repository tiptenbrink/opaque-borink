from opaquepy.opaquepy import _opqrust


def generate_keys() -> tuple[str, str]:
    """

    :return: Tuple of encoded private key and public key, respectiely.
    """
    return _opqrust.generate_keys_py()


def register(client_request: str, public_key: str) -> tuple[str, str]:
    """

    :param client_request:
    :param public_key:
    :return: Tuple of encoded response to the client and register state to be saved, respectively.
    """
    return _opqrust.register_server_py(client_request, public_key)


def register_finish(client_request_finish: str, register_state: str) -> str:
    """

    :param client_request_finish:
    :param register_state:
    :return: Password file to be saved.
    """
    return _opqrust.register_server_finish_py(client_request_finish, register_state)


def login(password_file: str, client_request: str, private_key: str) -> tuple[str, str]:
    """

    :param password_file:
    :param client_request:
    :param private_key:
    :return: Tuple of encoded response to the client and login state to be saved, respectively.
    """
    return _opqrust.login_server_py(password_file, client_request, private_key)


def login_finish(client_request_finish: str, login_state: str) -> str:
    """
    Finish the login process on the backend.

    :param client_request_finish: Client request to finish login, base64url-encoded.
    :param login_state: Saved login state from the previous step, base64url-encoded.
    :return: The session key, base64url-encoded.
    """
    return _opqrust.login_server_finish_py(client_request_finish, login_state)
