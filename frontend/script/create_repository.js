document.addEventListener('DOMContentLoaded', () => {
    console.log("Create repository page loaded.");
    console.log(repositories);
    const form = document.getElementById("create-repository-form");

    form.addEventListener("submit", (event) => {
        event.preventDefault();
        const repositoryName = document.getElementById("repository-name").textContent;
        const repositoryDescription = document.getElementById("repository-description").textContent;
        const isPublic = true;
    })
});