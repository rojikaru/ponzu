from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.genre import Genre
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import AdminPermission


class GenreViewSet(ViewSet):
    repository = Repository(Genre)
    permission_classes = [AdminPermission]

    async def list(self, request):
        genres = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(genres))

    async def retrieve(self, request, pk=None):
        genre = await self.repository.get_by_id(pk)
        if not genre:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(genre))

    async def create(self, request):
        genre = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(genre), status=201)

    async def partial_update(self, request, pk=None):
        genre = await self.repository.update(pk, **request.data)
        if not genre:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(genre))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
