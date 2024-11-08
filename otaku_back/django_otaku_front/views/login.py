from django.shortcuts import render, redirect
from django.views.generic import TemplateView
from django_otaku_front.forms.login import LoginForm
from django_otaku_front.request_session import create_session, get_session, set_access_token, get_full_url, delete_tokens

class LoginView(TemplateView):
    template_name = 'login.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['title'] = 'Login'
        context['description'] = 'Enter your credentials to log in.'
        context['form'] = LoginForm()
        return context

    def post(self, request, *args, **kwargs):
        form = LoginForm(request.POST)
        if form.is_valid():
            username = form.cleaned_data['username']
            password = form.cleaned_data['password']
            session_id = create_session()
            session = get_session(session_id)
            response = session.post(get_full_url('auth/login/'), data={'username': username, 'password': password})
            if response.status_code == 200:
                tokens = response.json()
                access_token = tokens.get('access_token')
                refresh_token = tokens.get('refresh_token')
                if refresh_token:
                    request.session['access_token'] = access_token
                    request.session['refresh_token'] = refresh_token
                    request.session['session_id'] = session_id
                    set_access_token(session, access_token)
                else:
                    delete_tokens(session)
        return render(request, self.template_name, {'form': form, 'title': 'Login', 'description': 'Please enter your credentials to log in.'})