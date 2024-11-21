from typing import List, Optional, Type, Dict, Any

import beanie.exceptions
from beanie import Document, PydanticObjectId
from django.core.exceptions import BadRequest
from pydantic import BaseModel


class Repository[T: Document]:
    def __init__(self, model: Type[T]):
        self._model = model

    async def get_all(self) -> List[T]:
        return await self._model.find().to_list()

    async def get_by_id(self, instance_id: PydanticObjectId) -> Optional[T]:
        try:
            return await self._model.get(instance_id)
        except beanie.exceptions.DocumentNotFound:
            return None

    async def create(self, **kwargs) -> T:
        # Check if the document exists
        if await self.exists(kwargs.get(self._get_primary_key())):
            raise BadRequest(f'{self._model.__name__} already exists')

        instance = self._model(**kwargs)
        await instance.insert()
        return instance

    async def update(self, instance_id: PydanticObjectId, **kwargs) -> Optional[T]:
        instance = await self.get_by_id(instance_id)
        if not instance:
            return None

        # Do not allow primary key changes
        if kwargs.get(self._get_primary_key()) is not None:
            raise BadRequest("Changing _id is prohibited")

        # Update fields
        for key, value in kwargs.items():
            setattr(instance, key, value)

        await instance.save()
        return instance

    async def delete(self, instance_id: PydanticObjectId) -> bool:
        instance = await self.get_by_id(instance_id)
        if not instance:
            return False

        await instance.delete()
        return True

    async def exists(self, instance_id: PydanticObjectId) -> bool:
        instance = await self.get_by_id(instance_id)
        return instance is not None

    async def aggregate(self, **kwargs) -> Dict[str, Any]:
        # For aggregation, you'll use the aggregation framework of MongoDB
        return await self._model.aggregate(kwargs).to_list()

    def _get_related_fields(self) -> List[str]:
        """
        Get a list of related fields (Reference fields).
        """
        related_fields = []
        for field in self._model.__annotations__:
            if isinstance(self._model.__annotations__[field], type) and issubclass(self._model.__annotations__[field], BaseModel):
                related_fields.append(field)
        return related_fields

    def _get_primary_key(self) -> str:
        """
        Get the primary key field name (MongoDB uses _id by default).
        """
        return "_id"

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
