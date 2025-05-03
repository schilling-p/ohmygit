import {API_BASE_URL} from "./common.js";

document.addEventListener('DOMContentLoaded', async () => {
    const user_email = localStorage.getItem("user_email");
    if (user_email) {
        try {
            console.log(`Loading user repositories with ${user_email}.`);
            await loadUserRepositories(user_email);
        } catch (err) {
            console.error("Error loading user repositories: ", err);
        }
    }
});

async function loadUserRepositories(user_email) {
    const response = await fetch(`${API_BASE_URL}/user_repositories`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({user_email: user_email}),
    });
    const data = await response.json();
    console.log(data);
}
async function loadUserOrganizations() {}
async function loadUserActivity() {}