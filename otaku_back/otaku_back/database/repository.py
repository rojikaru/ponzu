from typing import List, Optional
from django.db import models

class Repository[T: models.Model]:
    def __init__(self, model: type[T]):
        self._model = model

    def get_all(self) -> List[T]:
        return self._model.objects.all()

    def get_by_id(self, instance_id) -> Optional[T]:
        try:
            return self._model.objects.get(id=instance_id)
        except self._model.DoesNotExist:
            return None

    def create(self, **kwargs) -> T:
        return self._model.objects.create(**kwargs)

    def update(self, instance_id, **kwargs) -> Optional[T]:
        instance = self.get_by_id(instance_id)
        if instance:
            for key, value in kwargs.items():
                setattr(instance, key, value)
            instance.save()
            return instance
        return None

    def delete(self, instance_id) -> bool:
        instance = self.get_by_id(instance_id)
        if instance:
            instance.delete()
            return True
        return False