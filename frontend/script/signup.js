document.getElementById("signupForm").addEventListener("submit", async (event) => {
    event.preventDefault();

    const username = document.getElementById("name").value;
    const email = document.getElementById("email").value;
    const password = document.getElementById("password").value;
    const signupMessage = document.getElementById("signup-output");

    try {
        const response = await fetch(`/signup/`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ name: username, email: email, hashed_pw: password }),
        });

        let data = {};
        const contentType = response.headers.get("content-type");
        if (contentType && contentType.includes("application/json")) {
            data = await response.json();
        }
        console.log("data: ", data);

        if (response.status === 201 && data.type === "Signup") {
            signupMessage.textContent = "Account created successfully!";
            window.location.href = "/dashboard/";
        } else if (response.status === 409) {
            signupMessage.textContent = data.error || data.message || "User already exists.";
        } else {
            signupMessage.textContent = data.message || "An error occurred during sign up. Please try again.";
        }

    } catch (err) {
        console.error("Sign Up error: ", err);
        signupMessage.textContent = "A network error occurred. Please try again.";
    }
});
