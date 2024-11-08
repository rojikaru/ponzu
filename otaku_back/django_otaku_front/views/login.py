from django.shortcuts import render
from django.views.generic import TemplateView, RedirectView

from django_otaku_front.forms.login import LoginForm
from django_otaku_front.network.helper import login


class LoginView(TemplateView):
    template_name = 'login.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['form'] = LoginForm()
        context['title'] = 'Login'
        return context

    def post(self, request, *args, **kwargs):
        form = LoginForm(request.POST)
        if not form.is_valid():
            return render(request, self.template_name,
                          {'form': form, 'title': 'Login', 'description': 'Invalid credentials. Please try again.'})

        username = form.cleaned_data['username']
        password = form.cleaned_data['password']

        if login(request, username, password):
            return RedirectView.as_view(url='/')(request)
        else:
            return render(
                request,
                self.template_name,
                {'form': form, 'title': 'Login', 'description': 'Wrong credentials. Please try again.'}
            )
