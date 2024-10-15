from typing import Generic, TypeVar, List, Optional
from django.db import models

T = TypeVar('T', bound=models.Model)

class Repository(Generic[T]):
    def __init__(self, model: T):
        self._model = model

    def get_all(self) -> List[T]:
        return self._model.objects.all()

    def get_by_id(self, id: int) -> Optional[T]:
        try:
            return self._model.objects.get(id=id)
        except self._model.DoesNotExist:
            return None

    def create(self, **kwargs) -> T:
        return self._model.objects.create(**kwargs)

    def update(self, id: int, **kwargs) -> Optional[T]:
        instance = self.get_by_id(id)
        if instance:
            for key, value in kwargs.items():
                setattr(instance, key, value)
            instance.save()
            return instance
        return None

    def delete(self, id: int) -> bool:
        instance = self.get_by_id(id)
        if instance:
            instance.delete()
            return True
        return False