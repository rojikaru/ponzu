from djongo import models


class Producer(models.Model):
    mal_id = models.PositiveBigIntegerField(primary_key=True)
    name = models.TextField()
    type = models.TextField()
    url = models.URLField()

    def __str__(self):
        return f'{self.name} ({self.type}, {self.url})'
