from djongo import models


class Demographic(models.Model):
    mal_id = models.PositiveBigIntegerField(primary_key=True)
    name = models.TextField()
    type = models.TextField()
