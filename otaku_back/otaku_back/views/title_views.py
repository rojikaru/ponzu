from rest_framework import viewsets
from otaku_back.database.schemas.title import Anime, Manga
from otaku_back.database.schemas.serializers import AnimeSerializer, MangaSerializer

class AnimeViewSet(viewsets.ModelViewSet):
    queryset = Anime.objects.all()
    serializer_class = AnimeSerializer

class MangaViewSet(viewsets.ModelViewSet):
    queryset = Manga.objects.all()
    serializer_class = MangaSerializer