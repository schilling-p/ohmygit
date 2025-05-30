document.addEventListener('DOMContentLoaded', async () => {
    const repo_owner = document.getElementById("repo_owner").textContent;
    const repo_name = document.getElementById("repo_name").textContent;
    const dropdown_menu = document.getElementById("branch-dropdown-content");
    const dropdown_button = document.getElementById("branch-dropdown-button");

    dropdown_button.addEventListener("click", async (event) => {
        event.stopPropagation();
        const dropdown_is_visible = dropdown_menu.classList.toggle('dropdown-hidden') === false;
        if (dropdown_is_visible) {
            try {
                const response = await fetch(`/repos/${repo_owner}/${repo_name}/branches`);
                const json = await response.json();
                const branches = json.data.branches;
                dropdown_menu.innerHTML = "";

                branches.forEach(branch => {
                    const item = document.createElement("a");
                    item.textContent = branch;
                    item.href = `/repos/${repo_owner}/${repo_name}/branch/${branch}`;
                    item.classList.add("dropdown-item");
                    
                    dropdown_menu.appendChild(item);
                });
            } catch (err) {
                console.error("Error fetching branches: ", err);
            }
        }
    });

    dropdown_menu.addEventListener("click", (event) => {
        event.stopPropagation();
    })

    window.addEventListener("click", (event) => {
        if (!dropdown_menu.classList.contains('dropdown-hidden')) {
            dropdown_menu.classList.add('dropdown-hidden');
        }
    });
});