from typing import List, Optional

from django.core.exceptions import BadRequest
from django.db import models


class Repository[T: models.Model]:
    def __init__(self, model: type[T]):
        self._model = model

    def get_all(self) -> List[T]:
        return self._model.objects.all()

    def get_by_id(self, instance_id) -> Optional[T]:
        try:
            primary_key_arg = {self._get_primary_key(): instance_id}
            return self._model.objects.get(**primary_key_arg)
        except self._model.DoesNotExist:
            return None

    def create(self, **kwargs) -> T:
        # Check if the model exists
        if self.exists(kwargs.get(self._get_primary_key())):
            raise BadRequest(f'{self._model.__name__} already exists')

        related_fields = self._get_related_fields()

        # Separate related fields from other fields
        related_data = {k: kwargs.pop(k) for k in list(kwargs.keys()) if k in related_fields}

        instance = self._model.objects.create(**kwargs)

        # Handle foreign fields after creation
        for field, related_ids in related_data.items():
            if len(related_ids) == 0:
                continue

            # if the field is a many-to-many field
            field = getattr(self._model, field).field
            if isinstance(field, models.ManyToManyField):
                getattr(instance, field.name).set(related_ids)

            # if the field is a foreign key field
            else:
                setattr(instance, field, related_ids)

        instance.save()
        return instance

    def update(self, instance_id, **kwargs) -> Optional[T]:
        instance = self.get_by_id(instance_id)
        if not instance:
            return None

        related_fields = self._get_related_fields()

        # Separate related fields from other fields
        related_data = {k: kwargs.pop(k) for k in list(kwargs.keys()) if k in related_fields}

        # Update the non-related fields
        for key, value in kwargs.items():
            setattr(instance, key, value)

        instance.save()

        # Handle many-to-many updates
        for field, related_ids in related_data.items():
            if isinstance(getattr(self._model, field).field, models.ManyToManyField):
                getattr(instance, field).set(related_ids)

        instance.save()
        return instance

    def delete(self, instance_id) -> bool:
        instance = self.get_by_id(instance_id)
        if not instance:
            return False

        instance.delete()
        return True

    def exists(self, instance_id) -> bool:
        primary_key_arg = {self._get_primary_key(): instance_id}
        return self._model.objects.filter(**primary_key_arg).exists()

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
