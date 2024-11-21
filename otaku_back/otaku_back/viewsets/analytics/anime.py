from rest_framework import viewsets
from rest_framework.decorators import action
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.review import AnimeReview
from otaku_back.database.schemas.title import Anime
from otaku_back.env import ENVIRON


class AnimeAnalyticsViewSet(viewsets.ViewSet):
    anime_repository = Repository(Anime)
    review_repository = Repository(AnimeReview)

    api_url = ENVIRON('API_URL')
    available_stats = [
        {
            'name': 'most_popular',
            'friendly_name': 'Most Popular',
            'url': api_url + 'analytics/anime/most-popular',
        },
        {
            'name': 'top_rated',
            'friendly_name': 'Top Rated',
            'url': api_url + 'analytics/anime/top-rated',
        },
        {
            'name': 'avg_rating',
            'friendly_name': 'Average Rating',
            'url': api_url + 'analytics/anime/avg-rating',
        },
        {
            'name': 'most_watched',
            'friendly_name': 'Most Watched',
            'url': api_url + 'analytics/anime/most-watched',
        },
        {
            'name': 'most_liked',
            'friendly_name': 'Most Liked',
            'url': api_url + 'analytics/anime/most-liked',
        },
        {
            'name': 'most_disliked',
            'friendly_name': 'Most Disliked',
            'url': api_url + 'analytics/anime/most-disliked',
        },
        {
            'name': 'top_demographic',
            'friendly_name': 'Top Demographic',
            'url': api_url + 'analytics/anime/top-demographic',
        }
    ]

    # Returns the list of available stats
    def list(self, request):
        return Response(self.available_stats)

    # Returns the aggregated stats of the most popular anime by year
    @action(detail=False, methods=['get'], url_path='most-popular')
    def most_popular(self, request):
        return Response('most_popular')

    # Returns the aggregated stats of the top rated anime by year
    @action(detail=False, methods=['get'], url_path='top-rated')
    def top_rated(self, request):
        return Response('top_rated')

    # Returns the aggregated stats of the average rating of new titles by year
    @action(detail=False, methods=['get'], url_path='avg-rating')
    def avg_rating(self, request):
        return Response('avg_rating')

    # Returns the aggregated stats of the most watched anime by year
    @action(detail=False, methods=['get'], url_path='most-watched')
    def most_watched(self, request):
        return Response('most_watched')

    # Returns the aggregated stats of the most liked anime by year
    # Uses the like count from the reviews where the score 7 or higher
    @action(detail=False, methods=['get'], url_path='most-liked')
    def most_liked(self, request):
        return Response('most_liked')

    # Returns the aggregated stats of the most disliked anime by year
    # Uses the like count from the reviews where the score 4 or lower
    @action(detail=False, methods=['get'], url_path='most-disliked')
    def most_disliked(self, request):
        return Response('most_disliked')
