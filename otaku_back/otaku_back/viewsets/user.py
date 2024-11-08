from django.contrib.auth.models import AnonymousUser
from rest_framework import viewsets
from rest_framework.decorators import action
from rest_framework.response import Response
from otaku_back.database.repository import Repository
from otaku_back.database.schemas.serializers import UserSerializer
from otaku_back.database.schemas.user import User
from otaku_back.security.permissions import UserPermission


class UserViewSet(viewsets.ViewSet):
    serializer_class = UserSerializer
    repository = Repository(User)
    permission_classes = [UserPermission]

    @action(detail=False, methods=['get'], url_path='me')
    def me(self, request):
        if not request.user or request.user.is_anonymous:
            return Response(status=401)
        user = self.repository.get_by_id(request.user.pk)
        serializer = self.serializer_class(user)
        return Response(serializer.data)

    def list(self, request):
        users = self.repository.get_all()
        serializer = self.serializer_class(users, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        user = self.repository.get_by_id(pk)
        if not user:
            return Response(status=404)
        serializer = self.serializer_class(user)
        return Response(serializer.data)

    def create(self, request):
        user = self.repository.create(**request.data)
        serializer = self.serializer_class(user)
        return Response(serializer.data, status=201)

    def partial_update(self, request, pk=None):
        user = self.repository.update(pk, **request.data)
        if not user:
            return Response(status=404)
        serializer = self.serializer_class(user)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
