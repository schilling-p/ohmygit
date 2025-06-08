document.addEventListener('DOMContentLoaded', () => {
    console.log("Create repository page loaded.");
    console.log(repositories);
    const form = document.getElementById("create-repository-form");

    form.addEventListener("submit", async (event) => {
        event.preventDefault();
        const repositoryName = document.getElementById("repository-name").value.trim();
        const repositoryDescription = document.getElementById("repository-description").value.trim();
        const isPublic = document.querySelector('input[name="is-public"]:checked').value === "true";

        const createRepositoryRequest = {
            repositoryName,
            description: repositoryDescription || null,
            isPublic,
        }

        try {
            const response = await fetch("/repos/create", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(createRepositoryRequest),
            });
            if (response.ok) {
                console.log("Repository created successfully.");
            }

        } catch (err) {
            console.error("Error creating repository: ", err);
        }
    });
});