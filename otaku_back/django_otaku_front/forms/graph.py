from django import forms


class GraphForm(forms.Form):
    min_year = forms.IntegerField(required=False)
    max_year = forms.IntegerField(required=False)
