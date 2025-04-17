import {API_BASE_URL} from "./common.js";

const check_status_button = document.getElementById("check-status")
const loginForm = document.getElementById("loginForm");
const statusText = document.getElementById("status-output");
check_status_button.addEventListener("click", async () => {
    try {
        const response = await fetch(`${API_BASE_URL}/health`);
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
        const response = await fetch(`${API_BASE_URL}/login`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({email: email, hashed_pw: password}),
        });

        const data = await response.json();

        if (response.ok) {
            statusText.textContent = "Account created successfully!";
            window.location.href = "dashboard.html";
        } else {
            statusText.textContent = data.message;
        }
    } catch (err) {
        console.error("Sign Up error: ", err);
        statusText.textContent = "An error occurred during sign up.";
    }
});