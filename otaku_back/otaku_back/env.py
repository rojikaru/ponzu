import environ
import os
from pathlib import Path

# Build paths inside the project like this: BASE_DIR / 'subdir'.
BASE_DIR = Path(__file__).resolve().parent.parent

# Bootstrap environment variables
env = environ.Env(
    DEBUG=(bool, False) # set default values and a typecast
)
environ.Env.read_env(os.path.join(BASE_DIR, '.env'))

# Export env
ENVIRON = env
