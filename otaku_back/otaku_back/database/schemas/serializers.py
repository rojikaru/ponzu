from datetime import datetime, timedelta
from uuid import uuid4

import jwt
from rest_framework import serializers
from rest_framework.exceptions import AuthenticationFailed

from .genre import Genre
from .demographic import Demographic
from .producer import Producer
from .title import Anime, Manga
from .review import AnimeReview, MangaReview

from .user import User
from ... import settings
from otaku_back.security.authentication import UserBackend


class JSONSerializerField(serializers.Field):
    """ Serializer for JSONField -- required to make field writable"""

    def to_internal_value(self, data):
        return data

    def to_representation(self, value):
        return value


class GenreSerializer(serializers.ModelSerializer):
    class Meta:
        model = Genre
        fields = '__all__'


class ProducerSerializer(serializers.ModelSerializer):
    class Meta:
        model = Producer
        fields = '__all__'


class DemographicSerializer(serializers.ModelSerializer):
    class Meta:
        model = Demographic
        fields = '__all__'


class AnimeSerializer(serializers.ModelSerializer):
    genres = GenreSerializer(many=True)
    demographics = DemographicSerializer(many=True)
    title_synonyms = JSONSerializerField()
    producers = ProducerSerializer(many=True)
    images = JSONSerializerField()

    class Meta:
        model = Anime
        fields = '__all__'


class MangaSerializer(serializers.ModelSerializer):
    genres = GenreSerializer(many=True)
    demographics = DemographicSerializer(many=True)
    title_synonyms = JSONSerializerField()
    producers = ProducerSerializer(many=True)
    images = JSONSerializerField()

    class Meta:
        model = Manga
        fields = '__all__'


class AnimeReviewSerializer(serializers.ModelSerializer):
    anime = serializers.PrimaryKeyRelatedField(queryset=Anime.objects.all())
    user = serializers.PrimaryKeyRelatedField(queryset=User.objects.all())

    class Meta:
        model = AnimeReview
        fields = '__all__'


class MangaReviewSerializer(serializers.ModelSerializer):
    manga = serializers.PrimaryKeyRelatedField(queryset=Manga.objects.all())
    user = serializers.PrimaryKeyRelatedField(queryset=User.objects.all())

    class Meta:
        model = MangaReview
        fields = '__all__'


class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = [
            '_id',
            'username',
            'email',
            'password',
            'image',
            'bio',
            'birth_date',
            'created_at',
            'updated_at'
        ]


class AuthSerializer(serializers.Serializer):
    username = serializers.CharField(required=False)
    email = serializers.EmailField(required=False)
    password = serializers.CharField(write_only=True, required=False)
    refresh_token = serializers.CharField(required=False, write_only=True)

    def validate_login(self, data):
        username = data.get('username')
        password = data.get('password')

        try:
            user = User.objects.get(email=username)
            username = user.username
        except User.DoesNotExist:
            username = username

        user_backend = UserBackend()
        auth_user = user_backend.authenticate(request=None, username=username, password=password)

        if auth_user is None:
            raise AuthenticationFailed('Invalid credentials')

        access_payload = {
            'user_id': str(auth_user._id),
            'username': auth_user.username,
            'exp': datetime.utcnow() + timedelta(minutes=15),
            'iat': datetime.utcnow()
        }
        refresh_payload = {
            'sub': str(auth_user._id) 
        }
        access_token = jwt.encode(access_payload, settings.SECRET_KEY, algorithm='HS256')
        refresh_token = jwt.encode(refresh_payload, settings.SECRET_KEY, algorithm='HS256')

        return {
            'access_token': access_token,
            'token_type': 'Bearer',
            'refresh_token': refresh_token
        }

    def validate_register(self, data):
        email = data.get('email')
        username = data.get('username')

        if User.objects.filter(email=email).exists():
            raise serializers.ValidationError("A user with this email already exists.")

        if User.objects.filter(username=username).exists():
            raise serializers.ValidationError("A user with this username already exists.")

        return data

    def create_user(self, validated_data):
        user = User.objects.create_user(
            email=validated_data['email'],
            username=validated_data['username'],
            password=validated_data['password'],
            image=validated_data.get('image', ''),
            bio=validated_data.get('bio', ''),
            birth_date=validated_data.get('birth_date', None)
        )
        return user

    def create_superuser(self, validated_data):
        user = User.objects.create_superuser(
            email=validated_data['email'],
            username=validated_data['username'],
            password=validated_data['password']
        )
        return user

    def validate_refresh(self, data):
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
            user = User.objects.get(_id=user_id)
        except User.DoesNotExist:
            raise AuthenticationFailed('User not found')

        access_payload = {
            'user_id': str(user._id),
            'username': user.username,
            'exp': datetime.utcnow() + timedelta(minutes=15),
            'iat': datetime.utcnow()
        }
        refresh_payload = {
            'sub': str(user._id)
        }

        access_token = jwt.encode(access_payload, settings.SECRET_KEY, algorithm='HS256')
        new_refresh_token = jwt.encode(refresh_payload, settings.SECRET_KEY, algorithm='HS256')

        return {
            'access_token': access_token,
            'token_type': 'Bearer',
            'refresh_token': new_refresh_token
        }