from rest_framework import viewsets
from otaku_back.database.schemas.review import AnimeReview, MangaReview
from otaku_back.database.schemas.serializers import AnimeReviewSerializer, MangaReviewSerializer

class AnimeReviewViewSet(viewsets.ModelViewSet):
    queryset = AnimeReview.objects.all()
    serializer_class = AnimeReviewSerializer

class MangaReviewViewSet(viewsets.ModelViewSet):
    queryset = MangaReview.objects.all()
    serializer_class = MangaReviewSerializer