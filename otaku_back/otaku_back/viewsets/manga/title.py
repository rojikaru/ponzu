from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.title import Manga
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import AdminPermission


class MangaViewSet(ViewSet):
    repository = Repository(Manga)
    permission_classes = [AdminPermission]

    async def list(self, request):
        mangas = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(mangas))

    async def retrieve(self, request, pk=None):
        manga = await self.repository.get_by_id(pk)
        if not manga:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(manga))

    async def create(self, request):
        manga = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(manga), status=201)

    async def partial_update(self, request, pk=None):
        manga = await self.repository.update(pk, **request.data)
        if not manga:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(manga))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
