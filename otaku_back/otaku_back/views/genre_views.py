from rest_framework import viewsets
from otaku_back.database.schemas.genre import Genre
from otaku_back.database.schemas.serializers import GenreSerializer

class GenreViewSet(viewsets.ModelViewSet):
    queryset = Genre.objects.all()
    serializer_class = GenreSerializer