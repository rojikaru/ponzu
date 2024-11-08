from django.shortcuts import render
from django.views.generic import TemplateView, RedirectView

from django_otaku_front.forms.register import RegisterForm
from django_otaku_front.network.helper import register


class RegisterView(TemplateView):
    template_name = 'register.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['form'] = RegisterForm()
        context['title'] = 'Register'
        return context

    def post(self, request, *args, **kwargs):
        form = RegisterForm(request.POST)
        if not form.is_valid():
            return render(
                request, 
                self.template_name, 
                {'form': form, 'title': 'Register', 'description': 'Invalid credentials, try again!'}
            )
        
        username = form.cleaned_data['username']
        password = form.cleaned_data['password']
        email = form.cleaned_data['email']
        
        if register(request, username, email, password):
            return RedirectView.as_view(url='/')(request)
        else:
            return render(
                request,
                self.template_name,
                {'form': form, 'title': 'Register', 'description': 'Please enter your details to register.'}
            )
