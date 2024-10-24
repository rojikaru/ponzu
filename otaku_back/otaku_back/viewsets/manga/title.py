from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.serializers import MangaSerializer
from otaku_back.database.schemas.title import Manga
from otaku_back.security.permissions import AdminPermission


class MangaViewSet(viewsets.ViewSet):
    serializer_class = MangaSerializer
    repository = Repository(Manga)
    permission_classes = [AdminPermission]

    def list(self, request):
        mangas = self.repository.get_all()
        serializer = self.serializer_class(mangas, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        manga = self.repository.get_by_id(pk)
        if not manga:
            return Response(status=404)
        serializer = self.serializer_class(manga)
        return Response(serializer.data)

    def create(self, request):
        manga = self.repository.create(**request.data)
        serializer = self.serializer_class(manga)
        return Response(serializer.data, status=201)


    def partial_update(self, request, pk=None):
        manga = self.repository.update(pk, **request.data)
        if not manga:
            return Response(status=404)
        serializer = self.serializer_class(manga)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
