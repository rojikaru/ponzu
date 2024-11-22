from adrf.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.review import MangaReview
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import UserPermission


class MangaReviewViewSet(ViewSet):
    repository = Repository(MangaReview)
    permission_classes = [UserPermission]

    async def list(self, request):
        manga_reviews = self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(manga_reviews))

    async def retrieve(self, request, pk=None):
        manga_review = self.repository.get_by_id(pk)
        if not manga_review:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(manga_review))

    async def create(self, request):
        manga_review = self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(manga_review), status=201)

    async def partial_update(self, request, pk=None):
        manga_review = self.repository.update(pk, **request.data)
        if not manga_review:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(manga_review))

    async def destroy(self, request, pk=None):
        if self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
