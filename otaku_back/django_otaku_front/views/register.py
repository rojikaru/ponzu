from django.shortcuts import render
from django.views.generic import TemplateView
from django_otaku_front.forms.register import RegisterForm
from django_otaku_front.network.request_session import create_session, get_session, set_access_token, get_full_url, delete_tokens

class RegisterView(TemplateView):
    template_name = 'register.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['title'] = 'Register'
        context['description'] = 'Create a new account.'
        context['form'] = RegisterForm()
        return context

    def post(self, request, *args, **kwargs):
        form = RegisterForm(request.POST)
        if form.is_valid():
            username = form.cleaned_data['username']
            password = form.cleaned_data['password']
            email = form.cleaned_data['email']
            session_id = create_session()
            session = get_session(session_id)
            response = session.post(get_full_url('auth/register/'), data={'username': username, 'password': password, 'email': email})
            if response.status_code == 200 or response.status_code == 201:
                tokens = response.json()
                access_token = tokens.get('access_token')
                refresh_token = tokens.get('refresh_token')
                print('access_token:', access_token)
                if refresh_token:
                    request.session['access_token'] = access_token
                    request.session['refresh_token'] = refresh_token
                    request.session['session_id'] = session_id
                    set_access_token(session, access_token)
                else:
                    delete_tokens(session)
        return render(request, self.template_name, {'form': form, 'title': 'Register', 'description': 'Please enter your details to register.'})