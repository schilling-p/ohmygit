document.getElementById("signupForm").addEventListener("submit", async (event) => {
    event.preventDefault();

    const username = document.getElementById("name").value.trim();
    const email = document.getElementById("email").value.trim();
    const password = document.getElementById("password").value.trim();
    const signupRequest = {
        username: username,
        email: email,
        hashed_pw: password,
    }
    const signupMessage = document.getElementById("signup-output");

    try {
        const response = await fetch(`/signup/`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(signupRequest),
        });

        const json = await response.json();
        console.log("response: ", json);

        if (response.status === 201 && json.type === "Signup") {
            signupMessage.textContent = "Account created successfully!";
            window.location.href = "/dashboard/";
        } else if (response.status === 409) {
            console.error("User already exists.");
        } else {
            signupMessage.textContent = json.message || "An error occurred during sign up. Please try again.";
        }

    } catch (err) {
        console.log(err);
        signupMessage.textContent = "A network error occurred. Please try again.";
    }
});
