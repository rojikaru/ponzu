import logging
import traceback

from django.core.exceptions import BadRequest
from django.http import JsonResponse, Http404
from rest_framework.exceptions import APIException


def api_exception_handler(exc, context):
    # Log the exception
    logging.error(exc)
    logging.error(traceback.format_exc())

    if isinstance(exc, APIException):
        status = exc.status_code
        message = exc.detail
    elif (isinstance(exc, BadRequest)
          or isinstance(exc, ValueError)
          or isinstance(exc, KeyError)
          or isinstance(exc, TypeError)):
        status = 400
        message = 'Bad Request'
    elif isinstance(exc, Http404):
        status = 404
        message = 'Not Found'
    else:
        status = 500
        message = 'Internal Server Error'

    return JsonResponse(
        data={
            'code': status,
            'message': message
        },
        status=status
    )
