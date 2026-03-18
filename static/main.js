const form = document.querySelector("#password-form");
const bar = document.querySelector("#strength-bar");

const colors = {
    "F": "#f82b2b",
    "D-": "#ff6f29",
    "D": "#e47d4a",
    "D+": "#ffc924",
    "C-": "#ffd033",
    "C": "#e4bf4f",
    "C+": "#aaf829",
    "B-": "#a7f12d",
    "B": "#8fc635",
    "B+": "#15e461",
    "A-": "#1fd662",
    "A": "#22c55e",
    "A+": "#26b55a"
}

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

            bar.style.width = `${data.score}%`;
            bar.style.background = colors[data.grade] || "gray";


        } catch (err) {
            console.error(err);
            document.querySelector("#results").textContent = "Error calling API";
        }
    } else {
        document.querySelector("#results").textContent = "Please enter your password.";
    }
});