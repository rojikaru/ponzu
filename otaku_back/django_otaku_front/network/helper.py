from otaku_back.env import ENVIRON

BASE_URL = ENVIRON('API_URL')


def get_full_url(endpoint):
    return f'{BASE_URL}{endpoint}'


def get_user(session):
    response = session.get(get_full_url('user/me'))
    print(get_full_url('user/me'), response)
    return None if response is None else response.json()


def get_anime_list(session):
    response = session.get(get_full_url('anime'))
    return response.json()