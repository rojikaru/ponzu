import json
import logging

from adrf.viewsets import ViewSet
from django.core.exceptions import BadRequest
from rest_framework.decorators import action
from rest_framework.response import Response

from otaku_back.database.repository import Repository
from otaku_back.database.schemas.title import Anime
from otaku_back.env import ENVIRON

import pandas as pd


class AnimeAnalyticsViewSet(ViewSet):
    repository = Repository(Anime)

    api_url = ENVIRON('API_URL')
    available_stats = [
        {
            'name': 'most-popular',
            'friendly_name': 'Most Popular',
            'url': api_url + 'analytics/anime/most-popular',
        },
        {
            'name': 'top-rated',
            'friendly_name': 'Top-Rated',
            'url': api_url + 'analytics/anime/top-rated',
        },
        {
            'name': 'avg-rating',
            'friendly_name': 'Average Rating',
            'url': api_url + 'analytics/anime/avg-rating',
        },
        {
            'name': 'titles',
            'friendly_name': 'Count of titles',
            'url': api_url + 'analytics/anime/titles',
        },
        {
            'name': 'popularity',
            'friendly_name': 'Annual popularity stats',
            'url': api_url + 'analytics/anime/popularity',
        },
        {
            'name': 'new-longest',
            'friendly_name': 'New longest anime',
            'url': api_url + 'analytics/anime/new-longest',
        },
    ]

    async def _inner_aggregate(self, body=None):
        try:
            if not body:
                return await self.repository.get_all()

            pipeline = json.loads(body.decode('utf-8'))
            if not isinstance(pipeline, list):
                raise BadRequest("Pipeline should be a list")

            return await self.repository.aggregate(pipeline)
        except json.JSONDecodeError:
            raise BadRequest("Invalid JSON in request body")
        except Exception as e:
            # Log the exact error for debugging
            logging.exception(f"Error in _inner_aggregate: {e}")
            raise BadRequest(str(e))

    # Returns the list of available stats
    async def list(self, request):
        return Response(self.available_stats)

    # Returns the aggregated stats of the average rating of new titles by year
    @action(detail=False, methods=['get'], url_path='avg-rating')
    async def avg_rating(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df.groupby('year').agg({'score': 'mean'}).reset_index()

        return Response(df.to_dict(orient='records'))

    # Returns the aggregated stats of the count of titles by year
    @action(detail=False, methods=['get'], url_path='titles')
    async def titles(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df.groupby('year').size().reset_index(name='count')

        return Response(df.to_dict(orient='records'))

    # Returns the aggregated stats of the popularity of titles by year
    @action(detail=False, methods=['get'], url_path='popularity')
    async def popularity(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df[df['popularity'].notnull()]
        df = df.groupby('year').agg({'popularity': 'mean'}).reset_index()

        return Response(df.to_dict(orient='records'))

    # Returns the aggregated stats of the top-rated (score>7.5) titles count by year
    @action(detail=False, methods=['get'], url_path='top-rated')
    async def top_rated(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df[df['score'] > 7.5].groupby('year').size().reset_index(name='count')

        return Response(df.to_dict(orient='records'))

    # Returns the aggregated stats of the most popular titles count by year
    @action(detail=False, methods=['get'], url_path='most-popular')
    async def most_popular(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df[df['popularity'] < 1000].groupby('year').size().reset_index(name='count')

        return Response(df.to_dict(orient='records'))

    # Returns the aggregated stats of the new longest anime by year
    @action(detail=False, methods=['get'], url_path='new-longest')
    async def new_longest(self, request):
        matches = await self._inner_aggregate(request.body)
        if len(matches) == 0:
            return Response(status=404)

        df = pd.DataFrame(matches)
        df['year'] = pd.to_datetime(df['year'], format='%Y')
        df = df.sort_values('episodes', ascending=False).groupby('year').size().reset_index()

        return Response(df.to_dict(orient='records'))
