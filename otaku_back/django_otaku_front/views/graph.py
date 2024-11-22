from django.shortcuts import redirect
from django.views.generic import TemplateView
import plotly.graph_objects as go
from bokeh.plotting import figure
from bokeh.io import export_png
import io
from django.http import HttpResponse
from django.views import View
import requests
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
        context['title'] = f'Dashboard {kwargs["version"].upper()}'
        context['versions'] = get_dashboard_version_list(self.request.session.get('session_id'))
        context['version'] = kwargs['version']
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
        print(data)
        context['url'] = kwargs['graph']
        context['version'] = kwargs['version']

        # selected_start_year = self.request.GET.get('start_year')
        # selected_end_year = self.request.GET.get('end_year')
        # if selected_start_year and selected_end_year:
        #     selected_start_year = int(selected_start_year)
        #     selected_end_year = int(selected_end_year)
        #     context['selected_start_year'] = selected_start_year
        #     context['selected_end_year'] = selected_end_year
        #     data = {k: v for k, v in data.items() if selected_start_year <= k <= selected_end_year}
        # else:
        #     context['selected_start_year'] = list(data.keys())[0]
        #     context['selected_end_year'] = list(data.keys())[-1]
        #
        # # Ensure left data point is always bigger than the right one
        # sorted_data = sorted(data.items(), key=lambda x: x[0])
        # for i in range(1, len(sorted_data)):
        #     if sorted_data[i][1] > sorted_data[i - 1][1]:
        #         sorted_data[i] = (sorted_data[i][0], sorted_data[i - 1][1])
        #
        # values = [v for k, v in sorted_data]
        # context['max_value'] = max(values)
        # context['min_value'] = min(values)
        # context['median_value'] = np.median(values)
        # context['avg_value'] = np.mean(values)
        # context['years'] = sorted(data.keys())

        return context


class GraphImageView(View):
    def get(self, request, *args, **kwargs):
        url = kwargs['graph_url']
        response = requests.get(url)
        data = response.json()

        start_year = int(request.GET.get('start_year', min(data.keys())))
        end_year = int(request.GET.get('end_year', max(data.keys())))
        filtered_data = {k: v for k, v in data.items() if start_year <= int(k) <= end_year}

        years = list(filtered_data.keys())
        values = list(filtered_data.values())

        fig = go.Figure(data=go.Scatter(x=years, y=values, mode='lines+markers'))
        fig.update_layout(title='Linear Graph', xaxis_title='Year', yaxis_title='Value')

        buffer = io.BytesIO()
        fig.write_image(buffer, format='png')
        buffer.seek(0)
        image_png = buffer.getvalue()
        buffer.close()

        return HttpResponse(image_png, content_type='image/png')
