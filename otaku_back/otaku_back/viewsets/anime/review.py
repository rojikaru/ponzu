from rest_framework.viewsets import ViewSet
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.review import AnimeReview
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import UserPermission


class AnimeReviewViewSet(ViewSet):
    repository = Repository(AnimeReview)
    permission_classes = [UserPermission]

    async def list(self, request):
        anime_reviews = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(anime_reviews))

    async def retrieve(self, request, pk=None):
        anime_review = await self.repository.get_by_id(pk)
        if not anime_review:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(anime_review))

    async def create(self, request):
        anime_review = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(anime_review), status=201)

    async def partial_update(self, request, pk=None):
        anime_review = await self.repository.update(pk, **request.data)
        if not anime_review:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(anime_review))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
