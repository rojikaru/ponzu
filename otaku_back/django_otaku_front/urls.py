from django.urls import path

from .views.home import HomeView
from .views.login import LoginView
from .views.register import RegisterView
from .views.logout import logout_view

urlpatterns = [
    path('', HomeView.as_view(), name='home'),
    path('login/', LoginView.as_view(), name='login'),
    path('register/', RegisterView.as_view(), name='register'),
    path('logout/', logout_view, name='logout'),
]
