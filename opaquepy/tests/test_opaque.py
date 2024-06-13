from typing import Generator
import pytest

from opaquepy import *


@pytest.fixture(scope="module")
def server_setup() -> Generator[None, None, str]:
    yield create_setup()


def test_create_setup(server_setup: str):
    assert server_setup


password = "pass"


@pytest.fixture(scope="module")
def client_registration() -> Generator[None, None, tuple[str]]:
    response, client_state = register_client(password)
    yield response, client_state


def test_register(server_setup: str, client_registration: tuple[str]):
    response = client_registration[0]
    client_state = client_registration[1]
    server_response = register(server_setup, response, "someperson")
    client_final_response = register_client_finish(client_state, password, server_response)
    assert client_final_response


def test_login_start():
    setup = "C5HVEMyOKYglRys_3a58GHLeRM0oa_pjxSO6mu-WEnfIdOO5mE7GpCz_Z0xrntzbeMQI3GACQet9N_3lh1eaEWM18tqMDhUEJ_TwfSJNEXavKLc2DHlxWcd5Xd8aiPMJJ11dZmU76urlWHZw5xJuvDfLbdnt2tIj-fmY9PobZQg"
    client_message = "iIx1sW5pj2GlKnb4V1MRrCFmLTkW_hhyXQCGmczMZUJZ50HlQnVjuPyvv6oKpD7d0ZzPDbZcnliqDlN-GHdrwUpAvLRYzwk1earfTNefonfH9faSj7307IMRwrfGyOhZ"
    password_file = "wBtSZIhSPTwEY13yNT6nfWhj0WVRnhiqsnAYhUu7nj_5mmv-Trgm3DULYEZLwYhQaadsk8rI8n0PD1mZi8AL7517p9b5wisa4TxrNDyifHLUI_P09Re5KTf8CUr_0I6vMYOhCBl7WgItYfj1h-lAZU5E77fmsl-6l4MIZ5oYIKENNClbtgX9GYz1WkrZpJdeDdnAA5AI-0cfh3AX8UjpD46BhzwxkFDhtMra4vpRFQSAvu7gVzsZSDQJoqTcYXjy"
    # password 'clientiele'
    cred_id = "someperson"

    assert login(setup, password_file, client_message, cred_id)


def test_login_finish():
    client_message = "eP_3skqnrkJXs-AZpXqaxihP4EsF1ek8eDxH4ktDflExfEvNF99-oVZ24hkahw05v-rH9B-1WCs91dGteIMH9Q"
    state = "NjqD_19cS-XSDTfOdHj5g8wC1zMlTy-ydF4kiwKHUjbC9e_8CZb4pkLi9t0694FS_QgT6T_jktK1PbSoPqsTyas6rleW5Ttf0yA_a488YahFnBYHhsjXRWccL4Y6atjNeFcYOhc2f6t5oLU4p_FIA1eKQQFNRMXxEXEuHDl_DKanO3_4Iqs659gCx0IrOZoxnBDvxk8sHXNO19-gmzAVGgAQp-wERCoW3FP6h21PZwRA99AWSOe2YETc-VL_MtRT"

    assert login_finish(client_message, state)


