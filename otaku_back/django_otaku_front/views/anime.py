import json

from django.shortcuts import render, redirect
from django.views.generic import TemplateView

from django_otaku_front.forms.anime import AnimeCreateForm, AnimeEditForm
from django_otaku_front.network.helper import get_anime_list, get_anime, delete_anime, create_anime, update_anime


class AnimeView(TemplateView):
    template_name = 'anime/detail.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        anime = get_anime(self.request.session.get('session_id'), kwargs['pk'])
        context['anime'] = anime
        return context


def anime_delete(request, **kwargs):
    is_ok = delete_anime(request.session.get('session_id'), kwargs['pk'])
    return redirect('anime_list')


class AnimeListView(TemplateView):
    template_name = 'anime/list.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        response = get_anime_list(self.request.session.get('session_id'))
        if response:
            context['animes'] = response
        else:
            context['animes'] = None
        return context

class AnimeCreateView(TemplateView):
    template_name = 'anime/create.html'
    
    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['form'] = AnimeCreateForm()
        return context

    def post(self, request, *args, **kwargs):
        form = AnimeCreateForm(request.POST)
        if form.is_valid():
            create_anime(
                request.session.get('session_id'),
                form.cleaned_data
            )
            return redirect('anime_list')
        return render(request, 'anime/create.html', {'form': form})

class AnimeEditView(TemplateView):
    template_name = 'anime/edit.html'
    
    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        anime = get_anime(self.request.session.get('session_id'), kwargs['pk'])
        context['form'] = AnimeEditForm(anime)
        context['anime'] = anime
        return context

    def post(self, request, *args, **kwargs):
        form = AnimeEditForm(request.POST)
        if form.is_valid():
            update_anime(
                request.session.get('session_id'),
                kwargs['pk'],
                form.cleaned_data
            )
            return redirect('anime_list')
        return render(request, 'anime/edit.html', {'form': form})
        
