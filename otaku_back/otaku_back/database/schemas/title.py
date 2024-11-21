from typing import Annotated
from beanie import Document, Indexed

from .demographic import Demographic
from .genre import Genre
from .producer import Producer


# We'll follow the same structure as Jikan API
class Title(Document):
    mal_id: Annotated[int, Indexed(unique=True)]

    title: str
    title_english: str
    title_japanese: str
    title_synonyms: list[str]
    # titles: list[dict]

    type: str
    episodes: int
    genres: list[Genre]
    demographics: list[Demographic]
    synopsis: str

    # aired: dict
    # airing: bool
    status: str
    # duration: str
    producers: list[Producer]
    year: int

    score: float
    rank: int

    images: dict

    class Settings:
        collection = None

    def __str__(self):
        return f'{self.title} ({self.year}, ID: {self.mal_id})'


class Anime(Title):
    class Settings:
        name = "anime"


class Manga(Title):
    class Settings:
        name = "manga"
