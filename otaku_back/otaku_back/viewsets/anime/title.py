from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.serializers import AnimeSerializer
from otaku_back.database.schemas.title import Anime


class AnimeViewSet(viewsets.ViewSet):
    serializer_class = AnimeSerializer
    repository = Repository(Anime)

    def list(self, request):
        animes = self.repository.get_all()
        serializer = self.serializer_class(animes, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        anime = self.repository.get_by_id(pk)
        if not anime:
            return Response(status=404)
        serializer = self.serializer_class(anime)
        return Response(serializer.data)

    def create(self, request):
        anime = self.repository.create(**request.data)
        serializer = self.serializer_class(anime)
        return Response(serializer.data, status=201)

    def partial_update(self, request, pk=None):
        anime = self.repository.update(pk, **request.data)
        if not anime:
            return Response(status=404)
        serializer = self.serializer_class(anime)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
