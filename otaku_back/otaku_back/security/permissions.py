from rest_framework.permissions import BasePermission, SAFE_METHODS


class CreateUserPermission(BasePermission):
    def has_permission(self, request, view):
        if view.basename == 'auth':
            return True
        if request.method in SAFE_METHODS and view.basename != 'auth':
            return True

        return request.user and request.user.is_authenticated