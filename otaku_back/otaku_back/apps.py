import logging
import sys

from django.apps import AppConfig
import asyncio

from otaku_back.database.start import db_init


class MyAppConfig(AppConfig):
    name = 'otaku_back'

    def ready(self):
        # Set up logging
        logging.basicConfig(
            level=logging.WARNING,
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
            datefmt='%d-%b-%y %H:%M:%S',
            stream=sys.stdout
        )
        
        # Call init when the app is ready
        asyncio.create_task(db_init())
