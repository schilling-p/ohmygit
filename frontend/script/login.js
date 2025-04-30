import {API_BASE_URL} from "./common.js";

const check_status_button = document.getElementById("check-status")
const loginForm = document.getElementById("loginForm");
const statusText = document.getElementById("status-output");
const loginRoute = "login";
const healthRoute = "health";
check_status_button.addEventListener("click", async () => {
    try {
        const response = await fetch(`${API_BASE_URL}/${healthRoute}`);
        const data = await response.json();
        statusText.textContent = data.message
    } catch (error) {
        statusText.textContent = error.message
    }
});

loginForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    const email = document.getElementById("email").value;
    const password = document.getElementById("password").value;

    try {
        console.log("Sending login request to: ", `${API_BASE_URL}/users/${loginRoute}`);
        const response = await fetch(`${API_BASE_URL}/${loginRoute}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({email: email, password: password}),
        });

        const data = await response.json();
        console.log(data)

        if (response.ok) {
            statusText.textContent = "Account created successfully!";
            window.location.href = "dashboard.html";
        } else if (response.status === 409) {
            statusText.textContent = data.error || data.message || "Email already exists.";
        } else {
            statusText.textContent = data.message || "An error occurred during sign up. Please try again.";
        }

    } catch (err) {
        console.error("Log In error: ", err);
        statusText.textContent = "An error occurred during login .";
    }
});