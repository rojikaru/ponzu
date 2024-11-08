from django.views.generic import RedirectView

from django_otaku_front.network.session import delete_tokens

def logout_view(request):
    session_id = request.session.get('session_id')
    if session_id:
        delete_tokens(session_id)
    request.session.flush()
    request.user = None

    # Redirect to main page
    return RedirectView.as_view(url='/')(request)