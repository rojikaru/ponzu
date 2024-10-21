from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.demographic import Demographic
from otaku_back.database.schemas.serializers import DemographicSerializer


class DemographicViewSet(viewsets.ViewSet):
    serializer_class = DemographicSerializer
    repository = Repository(Demographic)

    def list(self, request):
        demographics = self.repository.get_all()
        serializer = self.serializer_class(demographics, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        demographic = self.repository.get_by_id(pk)
        if not demographic:
            return Response(status=404)
        serializer = self.serializer_class(demographic)
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
        demographic = self.repository.update(pk, **serializer.validated_data)
        if not demographic:
            return Response(status=404)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
