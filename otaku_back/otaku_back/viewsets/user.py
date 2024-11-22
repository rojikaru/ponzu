from adrf.viewsets import ViewSet
from rest_framework.decorators import action
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.user import User
from otaku_back.json.helper import JsonConverter
from otaku_back.security.permissions import UserPermission


class UserViewSet(ViewSet):
    repository = Repository(User)
    permission_classes = [UserPermission]

    @action(detail=False, methods=['get'], url_path='me')
    async def me(self, request):
        if not request.user or request.user.is_anonymous:
            return Response(status=401)
        user = await self.repository.get_by_id(request.user.pk)
        return Response(JsonConverter.convert_to_jsonable(user))

    async def list(self, request):
        users = await self.repository.get_all()
        return Response(JsonConverter.convert_to_jsonable(users))

    async def retrieve(self, request, pk=None):
        user = await self.repository.get_by_id(pk)
        if not user:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(user))

    async def create(self, request):
        user = await self.repository.create(**request.data)
        return Response(JsonConverter.convert_to_jsonable(user), status=201)

    async def partial_update(self, request, pk=None):
        user = await self.repository.update(pk, **request.data)
        if not user:
            return Response(status=404)
        return Response(JsonConverter.convert_to_jsonable(user))

    async def destroy(self, request, pk=None):
        if await self.repository.delete(pk):
            return Response(status=204)
        return Response(status=404)
