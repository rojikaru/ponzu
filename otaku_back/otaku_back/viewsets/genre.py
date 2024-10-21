from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.genre import Genre
from otaku_back.database.schemas.serializers import GenreSerializer


class GenreViewSet(viewsets.ViewSet):
    serializer_class = GenreSerializer
    repository = Repository(Genre)

    def list(self, request):
        genres = self.repository.get_all()
        serializer = self.serializer_class(genres, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        genre = self.repository.get_by_id(pk)
        if not genre:
            return Response(status=404)
        serializer = self.serializer_class(genre)
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
        genre = self.repository.update(pk, **serializer.validated_data)
        if not genre:
            return Response(status=404)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
