# OtakuRank

OtakuRank is an anime and manga library application that allows users to rate, comment on, and share fan content for their favorite series.

## Features

- Comprehensive anime and manga database
- User ratings and reviews
- Fan content sharing (fanart, fanfiction, etc.)
- Personalized recommendations
- Social features for connecting with other fans

## Tech Stack

- Backend: Django
- Database: MongoDB
- Frontend: Next.js
- State Management: Jotai
- Styling: Tailwind CSS
- API: Jikan API
- Deployment: Heroku

[//]: # (- API: AniList API)


## Prerequisites

- Python 3.12+
- Node.js 20+
- MongoDB

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/rojikaru/otaku-rank.git
   cd otaku-rank
   ```

2. Set up the backend:
   ```
   cd otaku_back
   python -m venv venv
   source venv/bin/activate  # On Windows use `venv\Scripts\activate`
   pip install -r requirements.txt
   ```

3. Set up the frontend:
   ```
   cd ../otaku_front
   npm install
   ```

4. Configure the environment variables:
   - Create a `.env` file in both the backend and frontend directories
   - Add necessary environment variables (database connection, API keys, etc.)

5. Run database migrations:
   ```
   cd ../otaku_back
   python manage.py migrate
   ```

## Running the Application

1. Start the Django backend:
   ```
   cd backend
   python manage.py runserver
   ```

2. In a new terminal, start the Next.js frontend:
   ```
   cd frontend
   npm run dev
   ```

3. Access the application at `http://localhost:3000` (and backend at port 8000)

## Contributing

We welcome contributions to OtakuRank! Please read our [Contributing Guide](CONTRIBUTING.md) for more information on how to get started.

## License

This project is licensed under the MIT License.

## Contact

For any questions or concerns, please open an issue on this repository or contact the maintainers at emails provided in contributors' profiles.

---

Happy coding, and may your favorite series always be top-ranked! 🌟

![Pythor](https://github.com/rojikaru/otaku-rank/blob/main/pythor.webp)
