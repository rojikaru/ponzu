from rest_framework.permissions import BasePermission, SAFE_METHODS

# TODO: Allow SAFE_METHODS even if authorization header keeps bad token

class AdminPermission(BasePermission):
    def has_permission(self, request, view):
        if request.method in SAFE_METHODS:
            return True

        if request.user and request.user.is_superuser:
            return True

        return False


class UserPermission(BasePermission):
    def has_permission(self, request, view):
        if request.method in SAFE_METHODS:
            return True

        if request.user and request.user.is_superuser:
            return True

        if request.user or request.user.is_authenticated:
            if view.basename == 'user' and view.action in ['retrieve', 'update', 'partial_update', 'destroy']:
                return str(request.user.id) == view.kwargs.get('pk')

            if view.basename in ['anime_review', 'manga_review'] and view.action in ['create', 'update', 'partial_update', 'destroy']:
                return str(request.user.id) == request.data.get('user')

        return False


class NoAuthPermission(BasePermission):
    def has_permission(self, request, view):
        if view.basename == 'auth':
            return True
        if request.method in SAFE_METHODS and view.basename != 'auth':
            return True

        return request.user and request.user.is_authenticated