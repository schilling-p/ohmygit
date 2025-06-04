document.addEventListener('DOMContentLoaded', async () => {
    const repoOwner = document.getElementById("repo_owner").textContent;
    const repoName = document.getElementById("repo_name").textContent;

    const dropdownMenu = document.getElementById("branch-dropdown-content");
    const branchDropdownButton = document.getElementById("branch-dropdown-button");

    const newBranchDropdownButton = document.getElementById("new-branch-dropdown-button");
    const newBranchDropdownMenu = document.getElementById("new-branch-dropdown-content");
    const createBranchButton = document.getElementById("create-branch-button");
    const baseBranchSelector = document.getElementById("base-branch-selector");

    const repository_branches = [];
    try {
        const response = await fetch(`/repos/${repoOwner}/${repoName}/branches`);
        const json = await response.json();
        const branches = json.data.branches;
        repository_branches.push(...branches);
    } catch (err) {
        console.error("Error fetching branches: ", err);
    }

    repository_branches.forEach(branch => {
        const item = document.createElement("a");
        item.textContent = branch;
        item.href = `/repos/${repoOwner}/${repoName}/branch/${branch}`;
        item.classList.add("dropdown-item");

        dropdownMenu.appendChild(item);
    });

    repository_branches.forEach(branch => {
        const item = document.createElement("option");
        item.textContent = branch;
        item.classList.add("dropdown-item");

        baseBranchSelector.appendChild(item);
    });

    branchDropdownButton.addEventListener("click", (event) => {
        event.stopPropagation();
        dropdownMenu.classList.toggle('dropdown-hidden');
    });

    dropdownMenu.addEventListener("click", (event) => {
        event.stopPropagation();
    })

    newBranchDropdownButton.addEventListener("click", (event) => {
        event.stopPropagation()
        newBranchDropdownMenu.classList.toggle('dropdown-hidden');
    });

    window.addEventListener("click", (event) => {
        if (!dropdownMenu.classList.contains('dropdown-hidden')) {
            dropdownMenu.classList.add('dropdown-hidden');
        }

        if (!baseBranchSelector.classList.contains('dropdown-hidden')) {
            baseBranchSelector.classList.add('dropdown-hidden');
        }
    });

    createBranchButton.addEventListener("click", (event) => {
        event.stopPropagation();
        const newBranchName = document.getElementById("new-branch-name").value;
        const baseBranch = document.getElementById("base-branch").value;

        if (!newBranchName || !baseBranch) {
            alert("Please enter a valid branch name and choose a base branch.");
            return;
        }

        try {
            const response = fetch(`/repos/${repoOwner}/${repoName}/branches`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    new_branch_name: newBranchName,
                    base_branch_name: baseBranch,
                    switch_head: false
                }),
            });
            if (!response.ok) {
                console.log("Error creating branch: ", response);
            }
        } catch (err) {
            console.error("Error creating branch: ", err);
        }
    })
});