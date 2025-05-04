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
        console.log("Sending login request to: ", `${API_BASE_URL}/${loginRoute}`);
        const response = await fetch(`${API_BASE_URL}/${loginRoute}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({email: email, password: password}),
        });

        const json = await response.json();
        console.log(json)

        if (response.status === 200 && json.type === "Login") {
            statusText.textContent = "Login successful!";
            localStorage.setItem("user_email", json.data.user_email);
            window.location.href = "dashboard.html";
        } else if (response.status === 401) {
            statusText.textContent = json.error || json.message || "Wrong Login Credentials.";
        } else {
            statusText.textContent = json.message || "An error occurred during sign up. Please try again.";
        }

    } catch (err) {
        console.error("Log In error: ", err);
        statusText.textContent = "An error occurred during login .";
    }
});