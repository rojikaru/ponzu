from django import forms

class RegisterForm(forms.Form):
    username = forms.CharField(max_length=100, required=True)
    password = forms.CharField(widget=forms.PasswordInput, required=True)
    email = forms.EmailField(required=True)