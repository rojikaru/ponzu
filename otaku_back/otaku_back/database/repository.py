from typing import List, Optional, Type, Dict, Any

import beanie.exceptions
from beanie import Document, PydanticObjectId
from django.core.exceptions import BadRequest
from pydantic import BaseModel
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
        if field == '_id':
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

        # Check if the document exists
        if await self.exists(kwargs.get('_id')):
            raise APIException(f'{self._model.__name__} already exists', code=409)

        instance = self._model(**kwargs)
        await instance.insert()
        return instance

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

    async def aggregate(self, **kwargs) -> List[Dict[str, Any]]:
        Repository._ensure_kwargs_have_no_id(kwargs)

        # For aggregation, you'll use the aggregation framework of MongoDB
        return await self._model.aggregate(kwargs).to_list()

    @staticmethod
    def _check_mongo_id(instance_id: PydanticObjectId):
        if not isinstance(instance_id, PydanticObjectId):
            raise BadRequest("Invalid ObjectId")

    @staticmethod
    def _ensure_kwargs_have_no_id(kwargs: Dict[str, Any]):
        if '_id' in kwargs:
            raise BadRequest("Cannot merge a new _id")

    def _get_related_fields(self) -> List[str]:
        """
        Get a list of related fields (Reference fields).
        """
        related_fields = []
        for field in self._model.__annotations__:
            if isinstance(self._model.__annotations__[field], type) and issubclass(self._model.__annotations__[field], BaseModel):
                related_fields.append(field)
        return related_fields

    async def _handle_related_fields(self, instance, related_data: Dict[str, Any]):
        # Beanie uses references for related fields, so we need to handle them accordingly
        for field, related_value in related_data.items():
            if isinstance(related_value, list):
                # Handle many-to-many relationships (e.g., storing references)
                related_model = self._model.__annotations__[field]
                related_instances = await related_model.find({"_id": {"$in": related_value}}).to_list()
                setattr(instance, field, related_instances)
            else:
                # Handle foreign keys (Reference to another document)
                related_model = self._model.__annotations__[field]
                related_instance = await related_model.get(related_value)
                setattr(instance, field, related_instance)

        await instance.save()
        return instance
