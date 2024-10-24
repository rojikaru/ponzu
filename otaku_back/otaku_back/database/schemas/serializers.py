from datetime import datetime, timedelta

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
    username = serializers.CharField()
    password = serializers.CharField(write_only=True)

    def validate(self, attrs):
        username = attrs.get('username')
        password = attrs.get('password')

        try:
            user = User.objects.get(email=username)
            username = user.username
        except User.DoesNotExist:
            username = username

        user_backend = UserBackend()
        auth_user = user_backend.authenticate(request=None, username=username, password=password)

        if auth_user is None:
            raise AuthenticationFailed('Invalid credentials')

        # JWT generation logic
        payload = {
            'user_id': str(auth_user._id), 
            'username': auth_user.username,
            'exp': datetime.utcnow() + timedelta(days=1),  
            'iat': datetime.utcnow()
        }
        token = jwt.encode(payload, settings.SECRET_KEY, algorithm='HS256')

        return {
            # 'refresh_token': None,
            'access_token': token
        }


class RegisterSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = ['username', 'email', 'password']
        extra_kwargs = {'password': {'write_only': True}}

    def validate(self, data):
        email = data.get('email', None)
        username = data.get('username', None)

        if User.objects.filter(email=email).exists():
            raise serializers.ValidationError("A user with this email already exists.")

        if User.objects.filter(username=username).exists():
            raise serializers.ValidationError("A user with this username already exists.")

        return data

    def create(self, validated_data):
        user = User.objects.create_user(
            email=validated_data['email'],
            username=validated_data['username'],
            password=validated_data['password'],
            image=validated_data.get('image', ''),
            bio=validated_data.get('bio', ''),
            birth_date=validated_data.get('birth_date', None)
        )
        return user