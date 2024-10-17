from djongo import models


class Genre(models.Model):
    mal_id = models.PositiveBigIntegerField(primary_key=True)
    name = models.TextField()
    type = models.TextField()

    def __str__(self):
        return f'{self.name} ({self.type})'
