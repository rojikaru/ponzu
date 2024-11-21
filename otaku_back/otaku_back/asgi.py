import os
import asyncio

from django.core.asgi import get_asgi_application
from django.conf import settings

from otaku_back.database.start import init

# Set the settings module before anything else
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'otaku_back.settings')

# Initialize the Django ASGI application
application = get_asgi_application()

# Run Beanie initialization and start the Django ASGI server
async def main():
    settings.configure()
    print(f"DJANGO_SETTINGS_MODULE is set to: {os.getenv('DJANGO_SETTINGS_MODULE')}")
    await init()
    return application

# Uvicorn will be responsible for calling this main() method, ensuring proper ASGI startup
if __name__ == "__main__":
    asyncio.run(main())

