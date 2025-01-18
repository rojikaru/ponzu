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

        # Check if the event loop is already running and schedule db_init
        loop = asyncio.get_event_loop()
        if loop.is_running():
            asyncio.ensure_future(db_init())  # This schedules the coroutine to run in the running loop
        else:
            loop.run_until_complete(db_init())  # If the loop isn't running, run it until completion
