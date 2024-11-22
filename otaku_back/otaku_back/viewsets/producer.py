from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.producer import Producer
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import AdminPermission


class ProducerViewSet(ViewSet):
    repository = Repository(Producer)
    permission_classes = [AdminPermission]

    async def list(self, request):
        producers = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(producers))

    async def retrieve(self, request, pk=None):
        producer = await self.repository.get_by_id(pk)
        if not producer:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(producer))

    async def create(self, request):
        producer = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(producer), status=201)

    async def partial_update(self, request, pk=None):
        producer = await self.repository.update(pk, **request.data)
        if not producer:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(producer))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
