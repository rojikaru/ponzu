from rest_framework import viewsets
from otaku_back.database.schemas.user import User
from otaku_back.database.schemas.serializers import UserSerializer

class UserViewSet(viewsets.ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer