from djongo import models

from .demographic import Demographic
from .genre import Genre


# We'll follow the same structure as Jikan API
class Title(models.Model):
    class Meta:
        abstract = True

    mal_id = models.PositiveBigIntegerField(primary_key=True)

    title = models.TextField()
    title_english = models.TextField()
    title_japanese = models.TextField()
    title_synonyms = models.ArrayField(model_container=str)
    # titles = models.ArrayField(models.JSONField)

    type = models.TextField()
    episodes = models.PositiveIntegerField()
    genres = models.ArrayField(model_container=Genre)
    demographics = models.ArrayField(model_container=Demographic)
    synopsis = models.TextField()

    # aired = models.JSONField()
    # airing = models.BooleanField()
    status = models.TextField()
    # duration = models.TextField()
    producers = models.JSONField()
    year = models.PositiveIntegerField()

    score = models.FloatField()
    rank = models.PositiveIntegerField()

    images = models.JSONField()


class Anime(Title):
    pass


class Manga(Title):
    pass
