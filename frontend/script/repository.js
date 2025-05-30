document.addEventListener('DOMContentLoaded', async () => {
    const repo_owner = document.getElementById("repo_owner").textContent;
    const repo_name = document.getElementById("repo_name").textContent;
    const dropdown_menu = document.getElementById("branch-dropdown-menu");

    document.getElementById("branch-dropdown-toggle").addEventListener("click", async () => {
        if (dropdown_menu.style.display === "block") {
            dropdown_menu.style.display = "none";
            return;
        }

        const response = await fetch(`/repos/${repo_owner}/${repo_name}/branches`);
        if (!response.ok) {
            console.log("Error fetching branches");
            return;
        }

        const json = await response.json();
        const branches = json.data.branches;
        dropdown_menu.innerHTML = "";

        branches.forEach(branch => {
            const li = document.createElement("li");
            li.textContent = branch;
            dropdown_menu.appendChild(li);
        });

        dropdown_menu.style.display = "block";
    })
});