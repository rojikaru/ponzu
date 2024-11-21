from typing import Annotated

from beanie import Document, Indexed


class Producer(Document):
    mal_id: Annotated[int, Indexed(unique=True)]
    name: str
    type: str
    url: str

    class Settings:
        name: str = "producer"

    def __str__(self):
        return f'{self.name} ({self.type}, {self.url})'
