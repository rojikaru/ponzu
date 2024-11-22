from adrf.viewsets import ViewSet
from rest_framework import status
from rest_framework.decorators import action
from rest_framework.response import Response

from otaku_back.security.authentication import UserBackend
from otaku_back.security.permissions import NoAuthPermission


class AuthViewSet(ViewSet):
    permission_classes = [NoAuthPermission]
    uback = UserBackend()

    @action(detail=False, methods=['post'], url_path='login')
    async def login(self, request):
        return Response(
            await self.uback.validate_login(request.data),
            status=status.HTTP_200_OK
        )

    @action(detail=False, methods=['post'], url_path='register')
    async def register(self, request):
        validated_data = await self.uback.validate_register(request.data)
        user = await self.uback.create_user(validated_data)
        login = await self.uback.validate_login({
            'username': user.username,
            'password': request.data['password']
        })
        return Response(
            login,
            status=status.HTTP_201_CREATED
        )

    @action(detail=False, methods=['post'], url_path='refresh')
    async def refresh(self, request):
        return Response(
            await self.uback.validate_refresh(request.data),
            status=status.HTTP_200_OK
        )

    @action(detail=False, methods=['post'], url_path='create_superuser')
    async def create_superuser(self, request):
        validated_data = await self.uback.validate_register(request.data)
        user = await self.uback.create_superuser(validated_data)
        tokens = await self.uback.validate_login({
            'username': user.username,
            'password': request.data['password']
        })
        return Response(
            tokens,
            status=status.HTTP_201_CREATED
        )
