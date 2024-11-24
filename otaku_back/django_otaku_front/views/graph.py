import datetime

from bokeh.embed import components
from django.shortcuts import redirect
from django.views.generic import TemplateView
import plotly.graph_objects as go
from bokeh.plotting import figure
import io
from django.http import HttpResponse
from django.views import View
import numpy as np

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

        data = get_anime_graph_data(
            self.request.session.get('session_id'),
            kwargs['graph'],
            # pipeline=[
            # {"$match": {"age": {"$gte": 18}}},  # Match users with age >= 18
            # {"$group": {"_id": "$city", "average_age": {"$avg": "$age"}}},
            # Group by city and calculate average age
            # {"$sort": {"average_age": -1}},  # Sort by average_age descending
            # ]
        )
        if not data:
            context['title'] = 'No data available'
            return context

        k, v = data[0].keys()
        keys = np.array([x[k] for x in data])
        values = np.array([x[v] for x in data])
        context['max_value'] = np.max(values)
        context['min_value'] = np.min(values)
        context['avg_value'] = np.mean(values)
        context['median_value'] = np.median(values)

        if kwargs['version'] == 'v2':
            keys = np.array([
                datetime.datetime.strptime(x[k], '%Y-%m-%dT%H:%M:%S').year
                for x in data
            ])

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
        keys = np.array([
            datetime.datetime.strptime(x[k], '%Y-%m-%dT%H:%M:%S').year
            for x in data
        ])
        values = np.array([x[v] for x in data])

        fig = go.Figure(data=go.Scatter(x=keys, y=values, mode='lines+markers'))
        fig.update_layout(xaxis_title='Year', yaxis_title='Value')

        img_bytes = fig.to_image(format='png')

        return HttpResponse(img_bytes, content_type='image/png')
