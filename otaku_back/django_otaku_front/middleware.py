from django_otaku_front.network.helper import get_user


class TokenAuthMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        session_id = request.session.get('session_id')
        if session_id:
            request.user = get_user(session_id)

        # Process the request
        return self.get_response(request)
