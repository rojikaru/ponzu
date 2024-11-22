import os

from django.core.asgi import get_asgi_application

# Set the settings module before anything else
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'otaku_back.settings')

# Initialize the Django ASGI application
application = get_asgi_application()
