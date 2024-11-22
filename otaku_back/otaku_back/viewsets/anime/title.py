from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.serializers import AnimeSerializer
from otaku_back.database.schemas.title import Anime
from otaku_back.security.permissions import AdminPermission
from otaku_back.json.helper import JsonConverter


class AnimeViewSet(ViewSet):
    repository = Repository(Anime)
    permission_classes = [AdminPermission]

    async def list(self, request):
        animes = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(animes))

    async def retrieve(self, request, pk=None):
        anime = await self.repository.get_by_id(pk)
        if not anime:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(anime))

    async def create(self, request):
        anime = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(anime), status=201)

    async def partial_update(self, request, pk=None):
        anime = await self.repository.update(pk, **request.data)
        if not anime:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(anime))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
