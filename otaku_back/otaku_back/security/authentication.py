from datetime import datetime, timedelta, UTC
from rest_framework import serializers
from rest_framework.exceptions import AuthenticationFailed
from otaku_back import settings
from bcrypt import checkpw
import jwt
from rest_framework import authentication, exceptions
from django.conf import settings
from otaku_back.database.repository import Repository
from otaku_back.database.schemas.user import User
from django.contrib.auth.backends import BaseBackend


class JWTAuthentication(authentication.BaseAuthentication):
    user_repository = Repository(User)

    async def authenticate(self, request):
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
            user = await self.user_repository.get_by_id(payload['sub'])
        except Exception:
            raise exceptions.AuthenticationFailed('User not found.')

        return user, None


class UserBackend(BaseBackend):
    user_repository = Repository(User)

    async def authenticate(self, request, username=None, password=None) -> User or None:
        try:
            user = await self.user_repository.find_by_field('username', username)
            password_matches = checkpw(password.encode(), user.password.encode())
            if user and password_matches:
                return user
            else:
                return None
        except User.DoesNotExist:
            return None

    async def validate_login(self, data) -> dict:
        username = data.get('username')
        password = data.get('password')

        try:
            user = await self.user_repository.find_by_field('username', username)
            username = user.username
        except User.DoesNotExist:
            username = username

        user_backend = UserBackend()
        auth_user = await user_backend.authenticate(
            request=None,
            username=username,
            password=password
        )

        if auth_user is None:
            raise AuthenticationFailed('Invalid credentials')

        access_payload = {
            'sub': str(auth_user.id),
            'username': auth_user.username,
            'exp': datetime.now(UTC) + timedelta(minutes=15),
            'iat': datetime.now(UTC)
        }
        refresh_payload = {
            'sub': str(auth_user.id)
        }
        access_token = jwt.encode(access_payload, settings.SECRET_KEY, algorithm='HS256')
        refresh_token = jwt.encode(refresh_payload, settings.SECRET_KEY, algorithm='HS256')

        return {
            'access_token': access_token,
            'token_type': 'Bearer',
            'refresh_token': refresh_token
        }

    async def validate_register(self, data) -> dict:
        email = data.get('email')
        username = data.get('username')

        if await self.user_repository.find_by_field('email', email):
            raise serializers.ValidationError("A user with this email already exists.")

        if await self.user_repository.find_by_field('username', username):
            raise serializers.ValidationError("A user with this username already exists.")

        return data

    async def create_user(self, validated_data) -> User:
        user = await self.user_repository.create(
            username=validated_data['username'],
            email=validated_data['email'],
            password=User.hash_password(validated_data['password']),
            is_superuser=False
        )
        return user

    async def create_superuser(self, validated_data) -> User:
        user = await self.user_repository.create(
            username=validated_data['username'],
            email=validated_data['email'],
            password=User.hash_password(validated_data['password']),
            is_superuser=True
        )
        return user

    async def validate_refresh(self, data) -> dict:
        refresh_token = data.get('refresh_token')

        try:
            payload = jwt.decode(refresh_token, settings.SECRET_KEY, algorithms=['HS256'])
        except jwt.ExpiredSignatureError:
            raise AuthenticationFailed('Refresh token has expired')
        except jwt.InvalidTokenError:
            raise AuthenticationFailed('Invalid refresh token')

        user_id = payload.get('sub')
        if not user_id:
            raise AuthenticationFailed('Invalid refresh token')

        try:
            user = await self.user_repository.get_by_id(user_id)
        except User.DoesNotExist:
            raise AuthenticationFailed('User not found')

        access_payload = {
            'user_id': str(user.id),
            'username': user.username,
            'exp': datetime.now(UTC) + timedelta(minutes=15),
            'iat': datetime.now(UTC)
        }
        refresh_payload = {
            'sub': str(user.id)
        }

        access_token = jwt.encode(access_payload, settings.SECRET_KEY, algorithm='HS256')
        new_refresh_token = jwt.encode(refresh_payload, settings.SECRET_KEY, algorithm='HS256')

        return {
            'access_token': access_token,
            'token_type': 'Bearer',
            'refresh_token': new_refresh_token
        }
