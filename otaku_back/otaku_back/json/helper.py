import json

from bson import ObjectId
from pydantic import BaseModel


class JsonConverter:
    @staticmethod
    def convert_to_jsonable(data):
        if isinstance(data, list) and not isinstance(data, str):
            return list(map(JsonConverter.convert_to_jsonable, data))

        if isinstance(data, (int, float, str, bool)):
            return data

        return {
            key:
                value if isinstance(value, (int, float, str, bool))
                else JsonConverter.convert_to_jsonable(value) if isinstance(value, list)
                else JsonConverter.convert_to_jsonable(value) if isinstance(value, BaseModel)
                else str(value) if isinstance(value, ObjectId)
                else value
            for key, value in data.__dict__.items()
            if key != 'revision_id'
        }

    @staticmethod
    def convert_to_object(data, object_type):
        return json.loads(data, object_hook=object_type.from_json)

    @staticmethod
    def convert_to_object_list(data, object_type):
        return json.loads(data, object_hook=object_type.from_json_list)
