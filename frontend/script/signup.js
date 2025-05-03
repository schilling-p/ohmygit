import {API_BASE_URL} from "./common.js";
const signupRoute = "signup";
document.getElementById("signupForm").addEventListener("submit", async (event) => {
    event.preventDefault();

    const username = document.getElementById("name").value;
    const email = document.getElementById("email").value;
    const password = document.getElementById("password").value;
    const signupMessage = document.getElementById("signup-output");

    try {
        console.log("Sending sign up request to: ", `${API_BASE_URL}/${signupRoute}`);
        const response = await fetch(`${API_BASE_URL}/${signupRoute}`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ name: username, email: email, hashed_pw: password }),
        });

        let data = {};
        const contentType = response.headers.get("content-type");
        if (contentType && contentType.includes("application/json")) {
            data = await response.json();
        }
        console.log("data: ", data);

        if (response.ok) {
            signupMessage.textContent = "Account created successfully!";
            // window.location.href = "index.html";
        } else if (response.status === 409) {
            signupMessage.textContent = data.error || data.message || "Email already exists.";
        } else {
            signupMessage.textContent = data.message || "An error occurred during sign up. Please try again.";
        }

    } catch (err) {
        console.error("Sign Up error: ", err);
        signupMessage.textContent = "A network error occurred. Please try again.";
    }
});
