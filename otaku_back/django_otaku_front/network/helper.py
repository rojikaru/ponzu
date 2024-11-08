from django_otaku_front.network.session import set_session_auth, create_session, get_session
from otaku_back.env import ENVIRON

BASE_URL = ENVIRON('API_URL')


def get_full_url(endpoint):
    return f'{BASE_URL}{endpoint}'


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
    return response.json()

def get_anime_list(session_id):
    session = get_session(session_id)
    if session is None:
        return None

    response = session.get(get_full_url('anime'))
    return response.json()