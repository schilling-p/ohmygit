import {API_BASE_URL} from "./common.js";

document.addEventListener('DOMContentLoaded', async () => {
    console.log("DOM fully loaded and parsed");
});

async function loadUserRepositories(user_email) {
    const response = await fetch(`${API_BASE_URL}/user_repositories`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({user_email: user_email}),

    });

    const json = await response.json();

    if (response.status === 200) {
        return json.data.repositories;
    } else {
        throw new Error("Error loading user repositories:");
    }
}

async function loadUserOrganizations(user_email) {
    const response = await fetch(`${API_BASE_URL}/user_organizations`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({user_email: user_email}),

    });

    const json = await response.json();

    if (response.status === 200) {
        return json.data.organizations;
    } else {
        throw new Error("Error loading user repositories:");
    }
}

function populateRepositories(repos) {
    const repoList = document.getElementById("repositories-list");
    repoList.innerHTML = "";

    repos.forEach(repo => {
        const repoElement = document.createElement("div");
        repoElement.classList.add("repo-card");

        const repoLink = document.createElement("a");
        repoLink.href = `repository.html?username=${localStorage.username}&repository_name=${repo.name}`;
        repoLink.textContent = repo.name;
        repoLink.style.color = "#0366d6";

        repoElement.appendChild(repoLink);
        repoList.appendChild(repoElement);
    });
}

function populateOrganizations(orgas) {
    const orgaList = document.getElementById("organizations-list");
    orgaList.innerHTML = "";

    orgas.forEach(orga => {
        const repoElement = document.createElement("div");
        repoElement.classList.add("repo-card");

        const orgaLink = document.createElement("a");
        orgaLink.href = `repo.html?id=${orga.id}`;
        orgaLink.textContent = orga.name;
        orgaLink.style.color = "#0366d6";

        repoElement.appendChild(orgaLink);
        orgaList.appendChild(repoElement);
    });
}

async function loadRepositoryData(repo_name) {
    const response = await fetch(`${API_BASE_URL}/api/repos/${localStorage.username}/${repo_name}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({username: localStorage.getItem("username"), repository_name: repo_name}),

    });

    const json = await response.json();
    console.log(json);
}

async function loadUserActivity() {}