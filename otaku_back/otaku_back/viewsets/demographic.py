from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.demographic import Demographic
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import AdminPermission


class DemographicViewSet(ViewSet):
    repository = Repository(Demographic)
    permission_classes = [AdminPermission]

    async def list(self, request):
        demographics = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(demographics))

    async def retrieve(self, request, pk=None):
        demographic = await self.repository.get_by_id(pk)
        if not demographic:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(demographic))

    async def create(self, request):
        demographic = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(demographic), status=201)

    async def partial_update(self, request, pk=None):
        demographic = await self.repository.update(pk, **request.data)
        if not demographic:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(demographic))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
