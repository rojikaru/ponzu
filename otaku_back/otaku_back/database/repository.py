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
        return self._handle_related_fields(instance, related_data)
        

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
        self._handle_related_fields(instance, related_data)

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

    def _handle_related_fields(self, instance, related_data):
        # Handle foreign fields
        for field, related_ids in related_data.items():
            # if the field is a many-to-many field
            field_attr = getattr(self._model, field).field
            if isinstance(field_attr, models.ManyToManyField):
                getattr(instance, field).set(related_ids)

            # if the field is a foreign key field
            else:
                # Fetch related field
                related_model = self._model._meta.get_field(field).related_model
                related_pk = related_model._meta.pk.name
                related_obj = related_model.objects.get(**{related_pk: related_ids})
                setattr(instance, field, related_obj)

        instance.save()
        return instance
