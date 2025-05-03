import {API_BASE_URL} from "./common.js";

document.addEventListener('DOMContentLoaded', async () => {
    const user_email = localStorage.getItem("user_email");
    // TODO: change this to generate some display message that an error has occurred
    if (!user_email) return;
    try {
        const repositories = await loadUserRepositories(user_email);
        populateRepositories(repositories);
    } catch (err) {
        console.error("Error loading user repositories: ", err);
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
    return await response.json();
}

function populateRepositories(repos) {
    const repoList = document.getElementById("repositories-list");
    repoList.innerHTML = "";

    repos.forEach(repo => {
        const repoElement = document.createElement("div");
        repoElement.classList.add("repo-card");

        const repoLink = document.createElement("a");
        repoLink.href = `repo.html?id=${repo.id}`;
        repoLink.textContent = repo.name;
        repoLink.style.color = "#0366d6";

        repoElement.appendChild(repoLink);
        repoList.appendChild(repoElement);
    });
}
async function loadUserOrganizations() {}
async function loadUserActivity() {}