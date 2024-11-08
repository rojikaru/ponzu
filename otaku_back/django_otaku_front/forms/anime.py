import json
import re

from django import forms
from rest_framework.exceptions import ValidationError


class ArrayField(forms.Field):
    def __init__(self, *args, **kwargs):
        kwargs['widget'] = forms.TextInput()  # Choose TextInput or Textarea
        super().__init__(*args, **kwargs)

    # Convert list to comma-separated string for display in the input
    def prepare_value(self, value):
        if isinstance(value, list):
            return ', '.join(value)
        return value

    # Convert comma-separated string to list
    def to_python(self, value):
        if not value:
            return []
        if isinstance(value, list):
            return value

        return [item.strip() for item in re.split(r'[;,]\s*', value)]

    def validate(self, value):
        super().validate(value)
        if not isinstance(value, list):
            raise ValidationError("Invalid data type. Expected a list.")


class ImagesField(forms.Field):
    def __init__(self, *args, **kwargs):
        kwargs['widget'] = forms.Textarea()
        super().__init__(*args, **kwargs)

    def prepare_value(self, value):
        if isinstance(value, dict):
            return json.dumps(value, indent=4)
        return value

    def to_python(self, value):
        if not value:
            return {}
        if isinstance(value, dict):
            return value
        try:
            return json.loads(value)
        except json.JSONDecodeError as e:
            raise ValidationError("Invalid JSON format")

    def validate(self, value):
        super().validate(value)
        if not isinstance(value, dict):
            raise ValidationError("Invalid data type. Expected a dictionary.")
        for key in ["jpg", "webp"]:
            if key not in value:
                continue

            if not isinstance(value[key], dict):
                raise ValidationError(f"Invalid data type for {key}. Expected a dictionary.")


class AnimeCreateForm(forms.Form):
    mal_id = forms.IntegerField(min_value=1, required=True)
    title = forms.CharField(required=True)
    title_english = forms.CharField(required=False)
    title_japanese = forms.CharField(required=False)
    title_synonyms = ArrayField(required=False)
    type = forms.CharField(required=False)
    episodes = forms.IntegerField(min_value=1, required=True)
    synopsis = forms.CharField(widget=forms.Textarea, required=False)
    status = forms.CharField(required=True)
    year = forms.IntegerField(min_value=1900, max_value=2200, required=True)
    score = forms.FloatField(min_value=0, max_value=10, required=False)
    rank = forms.IntegerField(min_value=1, required=False)
    images = ImagesField(required=False)



class AnimeEditForm(forms.Form):
    title = forms.CharField(required=True)
    title_english = forms.CharField(required=False)
    title_japanese = forms.CharField(required=False)
    title_synonyms = ArrayField(required=False)
    type = forms.CharField(required=False)
    episodes = forms.IntegerField(min_value=1, required=True)
    synopsis = forms.CharField(widget=forms.Textarea, required=False)
    status = forms.CharField(required=True)
    year = forms.IntegerField(min_value=1900, max_value=2200, required=True)
    score = forms.FloatField(min_value=0, max_value=10, required=False)
    rank = forms.IntegerField(min_value=1, required=False)
    images = ImagesField(required=False)
