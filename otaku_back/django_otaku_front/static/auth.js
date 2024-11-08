async function logout() {
    const data = await fetch('/logout/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-CSRFToken': getCookie('csrftoken')
        },
    });

    if (!data.ok) {
        throw new Error('Failed to logout');
    }

    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    window.location.href = '/login/';
}

function getCookie(name) {
    const cookieValue =
        document.cookie.match('(^|;)\\s*' + name + '\\s*=\\s*([^;]+)');
    return cookieValue ? cookieValue.pop() : '';
}

document.addEventListener('DOMContentLoaded', () => {
    const loginButton = document.getElementById('login');
    const registerButton = document.getElementById('register');

    const refreshToken = localStorage.getItem('refresh_token');
    if (refreshToken) {
        loginButton.textContent = 'Logout';
        loginButton.addEventListener('click', logout);

        registerButton.style.display = 'none';
    } else {
        loginButton.textContent = 'Login';
        loginButton.addEventListener('click', function () {
            window.location.href = '/login/';
        });

        registerButton.addEventListener('click', function () {
            window.location.href = '/register/';
        });
    }
});