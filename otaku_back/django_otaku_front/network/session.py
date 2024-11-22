import requests
from uuid import uuid4

# from django.contrib.sessions.models import Session


# use sqlite3 to store session
session_store = {}


def create_session():
    session_id = str(uuid4())
    session = requests.Session()
    session_store[session_id] = session
    return session_id


def get_session(session_id):
    return session_store.get(session_id)


def set_session_auth(request, session_id, access_token, refresh_token):
    request.session['access_token'] = access_token
    request.session['refresh_token'] = refresh_token
    request.session['session_id'] = session_id

    session = get_session(session_id)
    session.headers.update({
        'Authorization': f'Bearer {access_token}'
    })


def delete_tokens(session_id):
    session = get_session(session_id)
    if session is None:
        return

    if 'Authorization' in session.headers:
        del session.headers['Authorization']
    if 'Refresh-Token' in session.headers:
        del session.headers['Refresh-Token']
