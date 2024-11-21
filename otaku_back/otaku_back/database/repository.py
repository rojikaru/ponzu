from typing import List, Optional, Type, Dict, Any

from django.core.exceptions import BadRequest
from django.db import models
from asgiref.sync import sync_to_async


class Repository[T: models.Model]:
    def __init__(self, model: Type[T]):
        self._model = model

    async def get_all(self) -> List[T]:
        return await sync_to_async(self._model.objects.all)()

    async def get_by_id(self, instance_id) -> Optional[T]:
        try:
            primary_key_arg = {self._get_primary_key(): instance_id}
            return await self._model.objects.aget(**primary_key_arg)
        except self._model.DoesNotExist:
            return None

    async def create(self, **kwargs) -> T:
        # Check if the model exists
        if await self.exists(kwargs.get(self._get_primary_key())):
            raise BadRequest(f'{self._model.__name__} already exists')

        related_fields = self._get_related_fields()

        # Separate related fields from other fields
        related_data = {k: kwargs.pop(k) for k in list(kwargs.keys()) if k in related_fields}

        instance = await self._model.objects.acreate(**kwargs)
        return await self._handle_related_fields(instance, related_data)

    async def update(self, instance_id, **kwargs) -> Optional[T]:
        if kwargs.get(self._get_primary_key()) is not None:
            raise BadRequest('Changing id is prohibited')

        instance = await self.get_by_id(instance_id)
        if not instance:
            return None

        related_fields = self._get_related_fields()

        # Separate related fields from other fields
        related_data = {k: kwargs.pop(k) for k in list(kwargs.keys()) if k in related_fields}

        # Update the non-related fields
        for key, value in kwargs.items():
            setattr(instance, key, value)

        await instance.asave()

        # Handle many-to-many updates
        await self._handle_related_fields(instance, related_data)

        await instance.asave()
        return instance

    async def delete(self, instance_id) -> bool:
        instance = await self.get_by_id(instance_id)
        if not instance:
            return False

        await instance.adelete()
        return True

    async def exists(self, instance_id) -> bool:
        primary_key_arg = {self._get_primary_key(): instance_id}
        return await self._model.objects.filter(**primary_key_arg).aexists()

    async def aggregate(self, **kwargs) -> Dict[str, Any]:
        return await self._model.objects.aaggregate(**kwargs)

    def _get_related_fields(self) -> List[str]:
        """
        Get a list of related fields (foreign key or many-to-many).
        """
        related_fields = []
        for field in self._model._meta.get_fields():
            if isinstance(field, (models.ForeignKey, models.ManyToManyField)):
                related_fields.append(field.name)
        return related_fields

    def _get_primary_key(self) -> str:
        """
        Get the primary key field name.
        """
        return self._model._meta.pk.name

    async def _handle_related_fields(self, instance, related_data):
        # Handle foreign fields
        for field, related_ids in related_data.items():
            # if the field is a many-to-many field
            field_attr = getattr(self._model, field).field
            if isinstance(field_attr, models.ManyToManyField):
                await getattr(instance, field).asave(related_ids)

            # if the field is a foreign key field
            else:
                # Fetch related field
                related_model = self._model._meta.get_field(field).related_model
                related_pk = related_model._meta.pk.name
                related_obj = await related_model.objects.aget(**{related_pk: related_ids})
                setattr(instance, field, related_obj)

        await instance.asave()
        return instance
