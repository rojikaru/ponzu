from django_otaku_front.network.session import set_session_auth, create_session, get_session
from otaku_back.env import ENVIRON

BASE_URL = ENVIRON('API_URL')


def get_full_url(endpoint):
    url = f'{BASE_URL}{endpoint}'
    if not url.endswith('/'):
        url += '/'
    return url


def login(request, username, password):
    session_id = create_session()
    session = get_session(session_id)

    response = session.post(
        get_full_url('auth/login/'),
        data={'username': username, 'password': password}
    )

    if not response.ok:
        return False

    response_json = response.json()
    set_session_auth(
        request,
        session_id,
        response_json['access_token'],
        response_json['refresh_token']
    )
    return True


def register(request, username, email, password):
    session_id = create_session()
    session = get_session(session_id)

    response = session.post(
        get_full_url('auth/register/'),
        data={'username': username, 'email': email, 'password': password}
    )

    if not response.ok:
        return False

    response_json = response.json()
    set_session_auth(
        request,
        response_json['session_id'],
        response_json['access_token'],
        response_json['refresh_token']
    )
    return True


def get_user(session_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(get_full_url('user/me'))
    if response.status_code >= 400:
        return None

    return response.json()


def get_anime(session_id, anime_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(get_full_url(f'anime/{anime_id}'))
    if response.status_code >= 400:
        return None

    return response.json()


def delete_anime(session_id, anime_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.delete(get_full_url(f'anime/{anime_id}'))
    return response.ok


def create_anime(session_id, data):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.post(get_full_url('anime'), json=data)
    if response.status_code >= 400:
        return None

    return response.json()


def update_anime(session_id, anime_id, data):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.patch(get_full_url(f'anime/{anime_id}'), json=data)
    if response.status_code >= 400:
        return None

    return response.json()


def get_anime_list(session_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(get_full_url('anime'))
    if response.status_code >= 400:
        return None

    return response.json()

def get_dashboard_version_list(session_id):
    return ['v1', 'v2']


def get_anime_graph_data(session_id, graph_slug, pipeline=None):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(
        get_full_url(f'analytics/anime/{graph_slug}'),
        json=pipeline
    )
    if response.status_code >= 400:
        return None

    return response.json()


def get_anime_graph_list(session_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(get_full_url(f'analytics/anime'))
    if response.status_code >= 400:
        return None

    return response.json()    