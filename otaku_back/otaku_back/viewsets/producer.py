from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.producer import Producer
from otaku_back.database.schemas.serializers import ProducerSerializer
from otaku_back.security.permissions import AdminPermission


class ProducerViewSet(viewsets.ViewSet):
    serializer_class = ProducerSerializer
    repository = Repository(Producer)
    permission_classes = [AdminPermission]

    def list(self, request):
        producers = self.repository.get_all()
        serializer = self.serializer_class(producers, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        producer = self.repository.get_by_id(pk)
        if not producer:
            return Response(status=404)
        serializer = self.serializer_class(producer)
        return Response(serializer.data)

    def create(self, request):
        producer = self.repository.create(**request.data)
        serializer = self.serializer_class(producer)
        return Response(serializer.data, status=201)

    def partial_update(self, request, pk=None):
        producer = self.repository.update(pk, **request.data)
        if not producer:
            return Response(status=404)
        serializer = self.serializer_class(producer)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
