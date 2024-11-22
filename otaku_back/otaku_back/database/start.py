import motor.motor_asyncio
from beanie import init_beanie

from otaku_back.database.schemas.demographic import Demographic
from otaku_back.database.schemas.genre import Genre
from otaku_back.database.schemas.producer import Producer
from otaku_back.database.schemas.title import Anime, Manga
from otaku_back.database.schemas.review import AnimeReview, MangaReview
from otaku_back.database.schemas.user import User
from otaku_back.env import ENVIRON


# Initialize Beanie with the database and your models
async def db_init():
    client = motor.motor_asyncio.AsyncIOMotorClient(ENVIRON('DATABASE_URL'))
    await init_beanie(
        database=client[ENVIRON('DATABASE_NAME')],
        document_models=[
            # Add models here
            Demographic,
            Genre,
            Producer,
            Anime, Manga,
            AnimeReview, MangaReview,
            User,
        ]
    )
