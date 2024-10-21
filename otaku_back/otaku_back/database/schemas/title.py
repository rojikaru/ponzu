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
    title_synonyms = models.JSONField()
    # titles = models.ArrayField(models.JSONField)

    type = models.TextField()
    episodes = models.PositiveIntegerField()
    genres = models.JSONField() # weak relationship
    demographics = models.JSONField()
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

    def __str__(self):
        return f'{self.title} ({self.year}, ID: {self.mal_id})'


class Anime(Title):
    pass


class Manga(Title):
    pass
