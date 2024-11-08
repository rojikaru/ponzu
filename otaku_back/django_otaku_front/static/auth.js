
function logout() {
    fetch('/logout/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-CSRFToken': getCookie('csrftoken')
        }
    })
        .then(response => response.json())
        .then(data => {
            if (data.message === 'Logged out successfully') {
                localStorage.removeItem('access_token');
                localStorage.removeItem('refresh_token');
                window.location.href = '/login/';
            } else {
                alert('Logout failed');
            }
        })
        .catch(() => {
            alert('Logout failed');
        });
}

function getCookie(name) {
    let cookieValue = null;
    if (document.cookie && document.cookie !== '') {
        const cookies = document.cookie.split(';');
        for (let i = 0; i < cookies.length; i++) {
            const cookie = cookies[i].trim();
            if (cookie.substring(0, name.length + 1) === (name + '=')) {
                cookieValue = decodeURIComponent(cookie.substring(name.length + 1));
                break;
            }
        }
    }
    return cookieValue;
}

document.addEventListener('DOMContentLoaded', function() {
    const loginButton = document.getElementById('login');
    const registerButton = document.getElementById('register');
    const refreshToken = localStorage.getItem('refresh_token');

    if (refreshToken) {
        loginButton.textContent = 'Logout';
        loginButton.addEventListener('click', function() {
            logout();
        });
        registerButton.style.display = 'none';
    } else {
        loginButton.textContent = 'Login';
        loginButton.addEventListener('click', function() {
            window.location.href = '/login/';
        });
        registerButton.addEventListener('click', function() {
            window.location.href = '/register/';
        });
    }
});