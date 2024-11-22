from typing import List, Optional, Type, Dict, Any

import beanie.exceptions
from beanie import Document, PydanticObjectId
from django.core.exceptions import BadRequest
from pymongo.errors import DuplicateKeyError
from rest_framework.exceptions import APIException


class Repository[T: Document]:
    def __init__(self, model: Type[T]):
        self._model = model

    async def get_all(self) -> List[T]:
        return await self._model.all().to_list()

    async def get_by_id(self, instance_id: PydanticObjectId) -> Optional[T]:
        Repository._check_mongo_id(instance_id)

        try:
            return await self._model.get(instance_id)
        except beanie.exceptions.DocumentNotFound:
            return None

    async def find_by_field(self, field: str, value: Any) -> Optional[T]:
        if field == 'id':
            Repository._check_mongo_id(value)

        try:
            return await self._model.find_one({field: value})
        except beanie.exceptions.DocumentNotFound:
            return None

    async def find(self, **kwargs) -> List[T]:
        Repository._ensure_kwargs_have_no_id(kwargs)
        return await self._model.find(kwargs).to_list()

    async def find_one(self, **kwargs) -> Optional[T]:
        Repository._ensure_kwargs_have_no_id(kwargs)
        return await self._model.find_one(kwargs)

    async def create(self, **kwargs) -> T:
        Repository._ensure_kwargs_have_no_id(kwargs)

        try:
            # Check if the document exists
            if await self.exists(kwargs.get('id')):
                raise APIException(f'{self._model.__name__} already exists', code=409)

            instance = self._model(**kwargs)
            await instance.insert()
            return instance
        except DuplicateKeyError as e:
            raise APIException(f'{self._model.__name__} already exists', code=409)

    async def update(self, instance_id: PydanticObjectId, **kwargs) -> Optional[T]:
        Repository._ensure_kwargs_have_no_id(kwargs)

        instance = await self.get_by_id(instance_id)
        if not instance:
            return None

        # Update fields
        for key, value in kwargs.items():
            setattr(instance, key, value)

        await instance.save()
        return instance

    async def delete(self, instance_id: PydanticObjectId) -> bool:
        Repository._check_mongo_id(instance_id)

        instance = await self.get_by_id(instance_id)
        if not instance:
            return False

        await instance.delete()
        return True

    async def exists(self, instance_id: PydanticObjectId) -> bool:
        try:
            Repository._check_mongo_id(instance_id)
        except BadRequest:
            return False

        instance = await self.get_by_id(instance_id)
        return instance is not None

    async def aggregate(self, pipeline: List) -> List[Dict[str, Any]]:
        # For aggregation, you'll use the aggregation framework of MongoDB
        return await self._model.aggregate(pipeline).to_list()

    @staticmethod
    def _check_mongo_id(instance_id: Any):
        if not PydanticObjectId.is_valid(instance_id):
            raise BadRequest(f"Invalid _id: {instance_id}")

    @staticmethod
    def _ensure_kwargs_have_no_id(kwargs: Dict[str, Any]):
        if 'id' in kwargs:
            raise BadRequest("Cannot merge a new _id")
    
