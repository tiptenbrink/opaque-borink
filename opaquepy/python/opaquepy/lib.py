from opaquepy import _opaquepy


def create_setup() -> str:
    """
    :return: Encoded server setup state.
    """
    return _opaquepy.create_setup_py()


def register(setup: str, client_request: str, credential_id: str) -> str:
    """

    :param setup:
    :param client_request:
    :param credential_id:
    :return: Encoded response to the client.
    """
    return _opaquepy.register_server_py(setup, client_request, credential_id)


def register_finish(client_request_finish: str) -> str:
    """

    :param client_request_finish:
    :return: Encoded password file to be saved.
    """
    return _opaquepy.register_server_finish_py(client_request_finish)


def register_client(password: str) -> tuple[str, str]:
    """

    :param password:
    :return: Tuple of encoded response to the server and register state to be saved, respectively.
    """
    return _opaquepy.register_client_py(password)


def register_client_finish(
    client_register_state: str, password: str, server_message: str
) -> str:
    """

    :param client_register_state:
    :param password:
    :param server_message:
    :return: Encoded response to the server.
    """
    return _opaquepy.register_client_finish_py(
        client_register_state, password, server_message
    )


def login(
    setup: str, password_file: str, client_request: str, credential_id: str
) -> tuple[str, str]:
    """

    :param setup:
    :param password_file:
    :param client_request:
    :param credential_id:
    :return: Tuple of encoded response to the client and login state to be saved, respectively.
    """
    return _opaquepy.login_server_py(
        setup, password_file, client_request, credential_id
    )


def login_finish(client_request_finish: str, login_state: str) -> str:
    """
    Finish the login process on the backend.

    :param client_request_finish: Client request to finish login, base64url-encoded.
    :param login_state: Saved login state from the previous step, base64url-encoded.
    :return: The session key, base64url-encoded.
    """
    return _opaquepy.login_server_finish_py(client_request_finish, login_state)
