from datetime import datetime
from typing import Optional
from uuid import UUID

from beanie import Document, before_event
from beanie.odm.actions import EventTypes
from pydantic import Field

from otaku_back.database.schemas.title import Anime, Manga
from otaku_back.database.schemas.user import User


class Review(Document):
    _id: UUID = Field(default_factory=UUID)
    user: User
    score: int
    content: str
    created_at: Optional[datetime] = None
    updated_at: Optional[datetime] = None

    @before_event(EventTypes.INSERT)
    def set_created_at(self):
        self.created_at = datetime.now()

    @before_event(EventTypes.UPDATE)
    def set_updated_at(self):
        self.updated_at = datetime.now()

    class Settings:
        collection = None

    def __str__(self):
        return f'{self.user} ({self.score}, {self.created_at})'


class AnimeReview(Review):
    class Settings:
        name = "anime_review"

    anime = Anime


class MangaReview(Review):
    class Settings:
        name = "manga_review"

    manga = Manga
