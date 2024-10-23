from djongo import models
import uuid


class Review(models.Model):
    class Meta:
        abstract = True

    _id = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)

    user = models.ForeignKey('User', on_delete=models.CASCADE)

    # 1-10, Validate in the related service
    score = models.PositiveSmallIntegerField()

    content = models.TextField()

    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)


class AnimeReview(Review):
    class Meta:
        db_table = 'AnimeReview'

    anime = models.ForeignKey('Anime', on_delete=models.CASCADE)


class MangaReview(Review):
    class Meta:
        db_table = 'MangaReview'

    manga = models.ForeignKey('Manga', on_delete=models.CASCADE)
