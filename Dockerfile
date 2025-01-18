# Use an official Python runtime as a parent image
FROM python:3.13-slim

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY ./otaku_back /app/

# Install the dependencies from requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Migrate stub database
RUN python manage.py makemigrations
RUN python manage.py migrate

# Expose the port that the app will run on
EXPOSE 8000

# Set the command to start the application using uvicorn
CMD ["uvicorn", "otaku_back.asgi:application", "--host", "0.0.0.0", "--reload"]
