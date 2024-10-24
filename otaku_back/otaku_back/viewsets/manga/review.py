from rest_framework import viewsets
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.review import MangaReview
from otaku_back.database.schemas.serializers import MangaReviewSerializer
from otaku_back.security.permissions import UserPermission


class MangaReviewViewSet(viewsets.ViewSet):
    serializer_class = MangaReviewSerializer
    repository = Repository(MangaReview)
    permission_classes = [UserPermission]

    def list(self, request):
        manga_reviews = self.repository.get_all()
        serializer = self.serializer_class(manga_reviews, many=True)
        return Response(serializer.data)

    def retrieve(self, request, pk=None):
        manga_review = self.repository.get_by_id(pk)
        if not manga_review:
            return Response(status=404)
        serializer = self.serializer_class(manga_review)
        return Response(serializer.data)

    def create(self, request):
        manga_review = self.repository.create(**request.data)
        serializer = self.serializer_class(manga_review)
        return Response(serializer.data, status=201)

    def partial_update(self, request, pk=None):
        manga_review = self.repository.update(pk, **request.data)
        if not manga_review:
            return Response(status=404)
        serializer = self.serializer_class(manga_review)
        return Response(serializer.data)

    def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
