import requests
from uuid import uuid4

BASE_URL = 'http://localhost:8000/api/'

session_store = {}

def create_session():
    session_id = str(uuid4())
    session = requests.Session()
    session_store[session_id] = session
    return session_id

def get_session(session_id):
    return session_store.get(session_id)

def set_access_token(session, token):
    session.headers.update({
        'Authorization': f'Bearer {token}'
    })

def delete_tokens(session):
    if 'Authorization' in session.headers:
        del session.headers['Authorization']
    if 'Refresh-Token' in session.headers:
        del session.headers['Refresh-Token']

def get_full_url(endpoint):
    return f'{BASE_URL}{endpoint}'