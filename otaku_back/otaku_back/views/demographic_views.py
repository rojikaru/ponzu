from rest_framework import viewsets
from otaku_back.database.schemas.demographic import Demographic
from otaku_back.database.schemas.serializers import DemographicSerializer

class DemographicViewSet(viewsets.ModelViewSet):
    queryset = Demographic.objects.all()
    serializer_class = DemographicSerializer