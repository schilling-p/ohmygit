const API_BASE_URL = "http://127.0.0.1:3001";

window.addEventListener("DOMContentLoaded", () => {
    const button = document.getElementById("check-status")
    button.addEventListener("click", async () => {
        try {
            const response = await fetch(`${API_BASE_URL}/health`);
            const data = await response.json();
            document.getElementById("status-output").textContent = data.message
        } catch (error) {
            document.getElementById("status-output").textContent = error.message
        }
    });
});

