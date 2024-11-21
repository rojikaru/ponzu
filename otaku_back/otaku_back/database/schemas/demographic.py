from typing import Annotated

from beanie import Document, Indexed


class Demographic(Document):
    mal_id: Annotated[int, Indexed(unique=True)]
    name: str
    type: str

    class Settings:
        name: str = "demographic"

    def __str__(self):
        return f'{self.name} ({self.type})'
