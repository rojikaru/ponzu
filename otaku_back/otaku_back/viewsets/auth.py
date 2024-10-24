from rest_framework import viewsets, status
from rest_framework.decorators import action
from rest_framework.response import Response
from otaku_back.database.schemas.serializers import AuthSerializer
from otaku_back.security.permissions import NoAuthPermission


class AuthViewSet(viewsets.ViewSet):
    permission_classes = [NoAuthPermission]

    @action(detail=False, methods=['post'], url_path='login')
    def login(self, request):
        serializer = AuthSerializer(data=request.data)
        if serializer.is_valid(raise_exception=True):
            return Response(serializer.validate_login(serializer.validated_data), status=status.HTTP_200_OK)

    @action(detail=False, methods=['post'], url_path='register')
    def register(self, request):
        serializer = AuthSerializer(data=request.data)
        if serializer.is_valid(raise_exception=True):
            validated_data = serializer.validate_register(serializer.validated_data)
            user = serializer.create_user(validated_data)
            return Response(serializer.validate_login({'username': user.username, 'password': request.data['password']}), status=status.HTTP_201_CREATED)

    @action(detail=False, methods=['post'], url_path='refresh')
    def refresh(self, request):
        serializer = AuthSerializer(data=request.data)
        if serializer.is_valid(raise_exception=True):
            return Response(serializer.validate_refresh(serializer.validated_data), status=status.HTTP_200_OK)

    @action(detail=False, methods=['post'], url_path='create_superuser')
    def create_superuser(self, request):
        serializer = AuthSerializer(data=request.data)
        if serializer.is_valid(raise_exception=True):
            validated_data = serializer.validate_register(serializer.validated_data)
            user = serializer.create_superuser(validated_data)
            return Response(serializer.validate_login({'username': user.username, 'password': request.data['password']}), status=status.HTTP_201_CREATED)