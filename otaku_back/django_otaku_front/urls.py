from django.urls import path

from .views.anime import AnimeListView, AnimeCreateView, AnimeEditView, AnimeView, anime_delete
from .views.graph import dashboard_redirect, DashboardViewSet, GraphViewSet
from .views.login import LoginView
from .views.logout import logout_view
from .views.register import RegisterView

urlpatterns = [
    # path('', HomeView.as_view(), name='home'),

    path('', AnimeListView.as_view(), name='home'),
    path('anime/', AnimeListView.as_view(), name='anime_list'),
    path('anime/create/', AnimeCreateView.as_view(), name='anime_create'),
    path('anime/<int:pk>/', AnimeView.as_view(), name='anime_detail'),
    path('anime/<int:pk>/delete/', anime_delete, name='anime_delete'),
    path('anime/<int:pk>/edit/', AnimeEditView.as_view(), name='anime_edit'),
    path('dashboard/', dashboard_redirect, name='dashboard'),
    path('dashboard/<str:version>/', DashboardViewSet.as_view(), name='dashboard_page'),
    path('dashboard/<str:version>/<str:graph>/', GraphViewSet.as_view(), name='dashboard_graph'),
    path('login/', LoginView.as_view(), name='login'),
    path('register/', RegisterView.as_view(), name='register'),
    path('logout/', logout_view, name='logout'),
]
