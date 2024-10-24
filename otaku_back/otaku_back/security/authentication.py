import jwt
from rest_framework import authentication, exceptions
from django.conf import settings
from otaku_back.database.schemas.user import User
from django.contrib.auth.backends import BaseBackend
from django.contrib.auth.hashers import check_password


class JWTAuthentication(authentication.BaseAuthentication):
    def authenticate(self, request):
        auth_header = authentication.get_authorization_header(request).split()

        if not auth_header or auth_header[0].lower() != b'bearer':
            return None

        if len(auth_header) != 2:
            raise exceptions.AuthenticationFailed('Invalid token header')

        try:
            token = auth_header[1]
            payload = jwt.decode(token, settings.SECRET_KEY, algorithms=['HS256'])
        except jwt.ExpiredSignatureError:
            raise exceptions.AuthenticationFailed('Token has expired.')
        except jwt.InvalidTokenError:
            raise exceptions.AuthenticationFailed('Invalid token.')

        try:
            user = User.objects.get(_id=payload['user_id'])
        except User.DoesNotExist:
            raise exceptions.AuthenticationFailed('User not found.')

        return user, None


class UserBackend(BaseBackend):
    def authenticate(self, request, username=None, password=None):
        try:
            user = User.objects.get(username=username)
            password_matches = check_password(password, user.password)
            if user and password_matches:
                return user
            else:
                raise Exception()
        except Exception as e:
            check_password(None, None)
            return None

    def get_user(self, user_id):
        try:
            return User.objects.get(pk=user_id)
        except User.DoesNotExist:
            return None
