const form = document.querySelector("#password-form");

form.addEventListener("submit", async (e) => {
    const password = document.querySelector("#password").value;
    e.preventDefault();
    if (password != "") {

        try {
            const res = await fetch(`/score`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ password: password })
            });

            const data = await res.json();

            document.querySelector("#results").textContent =
                `Score: ${data.score} | Grade ${data.grade}`;
        } catch (err) {
            console.error(err);
            document.querySelector("#results").textContent = "Error calling API";
        }
    } else {
        document.querySelector("#results").textContent = "Please enter your password.";
    }
});