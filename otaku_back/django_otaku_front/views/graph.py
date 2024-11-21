from django.shortcuts import redirect
from django.views.generic import TemplateView

from django_otaku_front.network.helper import get_dashboard_version_list, get_anime_graph_list, get_anime_graph


def dashboard_redirect(request):
    return redirect('/dashboard/v1/', permanent=True)


class DashboardViewSet(TemplateView):
    template_name = 'dashboard/dashboard.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['title'] = f'Dashboard {kwargs['version'].upper()}'
        context['versions'] = get_dashboard_version_list(self.request.session.get('session_id'))
        context['version'] = kwargs['version']
        context['graphs'] = get_anime_graph_list(self.request.session.get('session_id'), kwargs['version'])
        return context


class GraphViewSet(TemplateView):
    template_name = 'dashboard/graph.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['title'] = f'Dashboard {kwargs['version'].upper()}'
        context['versions'] = get_dashboard_version_list(self.request.session.get('session_id'))
        context['version'] = kwargs['version']
        context['graphs'] = get_anime_graph_list(self.request.session.get('session_id'), kwargs['version'])
        context['graph'] = get_anime_graph(self.request.session.get('session_id'), kwargs['version'], kwargs['graph'])
        return context