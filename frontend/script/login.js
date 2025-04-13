import {API_BASE_URL} from "./common.js";

const check_status_button = document.getElementById("check-status")
const loginForm = document.getElementById("login-form");
check_status_button.addEventListener("click", async () => {
    try {
        const response = await fetch(`${API_BASE_URL}/health`);
        const data = await response.json();
        document.getElementById("status-output").textContent = data.message
    } catch (error) {
        document.getElementById("status-output").textContent = error.message
    }
});

