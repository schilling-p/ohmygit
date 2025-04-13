import {API_BASE_URL} from "./common.js";

const signupMessage = document.getElementById("signup-message");
document.getElementById("signupForm").addEventListener("submit", async (event) => {
    event.preventDefault();
    const username = document.getElementById("name").value;
    const email = document.getElementById("email").value;
    const password = document.getElementById("password").value;

    try {
        const response = await fetch(`${API_BASE_URL}/new_user`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({name: username, email: email, hashed_pw: password}),
        });

        const data = await response.json();

        if (response.ok) {
            signupMessage.textContent = "Account created successfully!";
            window.location.href = "dashboard.html";
        } else {
            signupMessage.textContent = data.message;
        }
    } catch (err) {
        console.error("Sign Up error: ", err);
        signupMessage.textContent = "An error occurred during sign up.";
    }
});