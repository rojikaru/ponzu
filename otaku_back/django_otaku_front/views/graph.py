import datetime

import numpy as np
import plotly.graph_objects as go
from bokeh.embed import components
from bokeh.plotting import figure
from django.http import HttpResponse
from django.shortcuts import redirect
from django.views import View
from django.views.generic import TemplateView

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
        data = get_anime_graph_data(
            self.request.session.get('session_id'),
            kwargs['graph'],
            # Match users with age >= 18

            pipeline=[
                {
                    "$match": {
                        "year": year_matcher
                    }
                }
            ]
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

        years = np.array([x[k] for x in data])
        context['min_year'] = np.min(years)
        context['max_year'] = np.max(years)

        # create a form for the user to filter the data
        form = MyDynamicForm
        if form.is_valid():
            # Handle the form data
            selected_value = form.cleaned_data['dynamic_field']
            print(f"Selected: {selected_value}")

        context['form'] = {
            'min_year': min_year,
            'max_year': max_year,
        }

        if kwargs['version'] == 'v2':
            keys = np.array([x[k] for x in data])

            fig = figure(x_axis_label='Year', y_axis_label='Value')
            fig.line(keys, values, line_width=2)

            bokeh_script, bokeh_div = components(fig)
            context['bokeh_script'] = bokeh_script
            context['bokeh_div'] = bokeh_div

        return context


class GraphImageView(View):
    def get(self, request, *args, **kwargs):
        url = kwargs['graph_url']
        data = get_anime_graph_data(request.session.get('session_id'), url)
        k, v = data[0].keys()
        keys = np.array([x[k] for x in data])
        values = np.array([x[v] for x in data])

        fig = go.Figure(data=go.Scatter(x=keys, y=values, mode='lines+markers'))
        fig.update_layout(xaxis_title='Year', yaxis_title='Value')

        img_bytes = fig.to_image(format='png')

        return HttpResponse(img_bytes, content_type='image/png')
