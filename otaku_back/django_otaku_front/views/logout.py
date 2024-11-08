from django.http import JsonResponse
from django_otaku_front.network.session import delete_tokens

def logout_view(request):
    user_session = request.session.get('user_session')
    if user_session:
        delete_tokens(user_session)
    request.session.flush()
    return JsonResponse({'message': 'Logged out successfully'})