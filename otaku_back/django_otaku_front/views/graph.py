import numpy as np
import plotly.graph_objects as go
from bokeh.embed import components
from bokeh.plotting import figure
from django.http import HttpResponse
from django.shortcuts import redirect
from django.utils.http import url_has_allowed_host_and_scheme
from django.views import View
from django.views.generic import TemplateView

from django_otaku_front.forms.graph import GraphForm
from django_otaku_front.network.helper import get_dashboard_version_list, get_anime_graph_list, get_anime_graph_data


def dashboard_redirect(request):
    return redirect('/dashboard/v1/', permanent=True)


class DashboardViewSet(TemplateView):
    template_name = 'dashboard/dashboard.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['title'] = f'Dashboard {kwargs["version"].upper()}'
        context['versions'] = get_dashboard_version_list(self.request.session.get('session_id'))
        context['version'] = kwargs['version']
        context['graphs'] = get_anime_graph_list(self.request.session.get('session_id'))
        return context


class GraphViewSet(TemplateView):
    template_name = 'dashboard/graph.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['url'] = kwargs['graph']
        context['version'] = kwargs['version']
        context['version'] = kwargs['version']
        context['title'] = f'Dashboard {kwargs["version"].upper()}'

        context['versions'] = get_dashboard_version_list(self.request.session.get('session_id'))
        context['graphs'] = get_anime_graph_list(self.request.session.get('session_id'))
        if not context['graphs']:
            context['title'] = 'No graphs available'
            return context

        context['graph_title'] = next(
            (x['friendly_name'] for x in context['graphs'] if x['name'] == kwargs['graph']),
            'Graph'  # Default value if no match is found
        )

        min_year = self.request.GET.get('min_year')
        max_year = self.request.GET.get('max_year')

        year_matcher = dict()
        if min_year:
            # $gte is MongoDB syntax for greater than or equal to
            year_matcher['$gte'] = int(min_year)
        if max_year:
            # $lte is MongoDB syntax for less than or equal to
            year_matcher['$lte'] = int(max_year)

        pipeline = []
        if year_matcher:
            pipeline.append({
                "$match": {
                    "year": year_matcher
                }
            })

        data = get_anime_graph_data(
            self.request.session.get('session_id'),
            kwargs['graph'],
            pipeline=pipeline
        )
        if not data:
            context['title'] = 'No data available'
            return context

        k, v = data[0].keys()
        values = np.array([x[v] for x in data])
        context['max_value'] = np.max(values)
        context['min_value'] = np.min(values)
        context['avg_value'] = np.mean(values)
        context['median_value'] = np.median(values)

        context['years'] = [x[k] for x in data]
        years = np.array(context['years'])
        # create a form for the user to filter the data
        context['form'] = GraphForm(initial={'min_year': np.min(years).astype(int), 'max_year': np.max(years).astype(int)})

        if kwargs['version'] == 'v2':
            fig = figure(x_axis_label='Year', y_axis_label='Value')
            fig.line(years, values, line_width=2)

            bokeh_script, bokeh_div = components(fig)
            context['bokeh_script'] = bokeh_script
            context['bokeh_div'] = bokeh_div

        return context

    def post(self, request, *args, **kwargs):
        form = GraphForm(request.POST)
        if not form.is_valid():
            return self.render_to_response({'form': form, 'title': 'Custom Graph'})

        valid_versions = ['v1', 'v2']  # Example list of valid versions
        valid_graphs = [graph['name'] for graph in get_anime_graph_list(self.request.session.get('session_id'))]

        version = kwargs.get("version")
        graph = kwargs.get("graph")

        query_params = '?' + '&'.join([f'{x}={y}' for x, y in form.cleaned_data.items() if y])
        if version in valid_versions and graph in valid_graphs:
            target_url = f'/dashboard/{version}/{graph}/{query_params}'
            if url_has_allowed_host_and_scheme(target_url, allowed_hosts=None):
                return redirect(target_url)

        return redirect('/')


class GraphImageView(View):
    def get(self, request, *args, **kwargs):
        url = kwargs['graph_url']
        min_year = request.GET.get('min_year')
        max_year = request.GET.get('max_year')

        year_matcher = dict()
        if min_year:
            year_matcher['$gte'] = int(min_year)
        if max_year:
            year_matcher['$lte'] = int(max_year)
        pipeline = []
        if year_matcher:
            pipeline = [{
                "$match": {
                    "year": year_matcher
                }
            }]

        data = get_anime_graph_data(request.session.get('session_id'), url, pipeline)
        if data is None:
            return HttpResponse(status=404)

        k, v = data[0].keys()
        keys = np.array([x[k] for x in data])
        values = np.array([x[v] for x in data])

        fig = go.Figure(data=go.Scatter(x=keys, y=values, mode='lines+markers'))
        fig.update_layout(xaxis_title='Year', yaxis_title='Value')

        img_bytes = fig.to_image(format='png')

        return HttpResponse(img_bytes, content_type='image/png')
