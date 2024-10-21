from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.serializers import UserSerializer
from otaku_back.database.schemas.user import User


class UserViewSet(viewsets.ViewSet):
    serializer_class = UserSerializer
    repository = Repository(User)

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
        serializer = self.serializer_class(data=request.data)
        if not serializer.is_valid():
            return Response(serializer.errors, status=400)
        self.repository.create(**serializer.validated_data)
        return Response(serializer.data, status=201)

    def update(self, request, pk=None):
        serializer = self.serializer_class(data=request.data)
        if not serializer.is_valid():
            return Response(serializer.errors, status=400)
        user = self.repository.update(pk, **serializer.validated_data)
        if not user:
            return Response(status=404)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
