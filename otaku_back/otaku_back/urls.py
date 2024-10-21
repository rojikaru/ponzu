"""
URL configuration for otaku_back project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path, include
from rest_framework.routers import DefaultRouter
from .env import ENVIRON
from .views.demographic_views import DemographicViewSet
from .views.genre_views import GenreViewSet
from .views.review_views import AnimeReviewViewSet, MangaReviewViewSet
from .views.title_views import AnimeViewSet, MangaViewSet
from .views.user_views import UserViewSet

router = DefaultRouter()
router.register(r'demographic', DemographicViewSet)
router.register(r'user', UserViewSet)
router.register(r'anime', AnimeViewSet)
router.register(r'manga', MangaViewSet)
router.register(r'genre', GenreViewSet)
router.register(r'review/anime', AnimeReviewViewSet)
router.register(r'review/manga', MangaReviewViewSet)

# The `urlpatterns` list routes URLs to views.
urlpatterns = [
    path('api/', include(router.urls)),
]

if ENVIRON('DEBUG'):
    print('DEBUG MODE ENABLED')
    urlpatterns.append(path('admin/', admin.site.urls))