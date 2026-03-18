const form = document.querySelector("#password-form");
const bar = document.querySelector("#strength-bar");

// Helper to get CSS variable value
function getGradeColor(grade) {
    const map = {
        "F": "--grade-f",
        "D-": "--grade-d-minus",
        "D": "--grade-d",
        "D+": "--grade-d-plus",
        "C-": "--grade-c-minus",
        "C": "--grade-c",
        "C+": "--grade-c-plus",
        "B-": "--grade-b-minus",
        "B": "--grade-b",
        "B+": "--grade-b-plus",
        "A-": "--grade-a-minus",
        "A": "--grade-a",
        "A+": "--grade-a-plus"
    };
    const varName = map[grade];
    if (!varName) return null;
    return getComputedStyle(document.documentElement).getPropertyValue(varName).trim();
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
            bar.style.background = getGradeColor(data.grade) || "gray";


        } catch (err) {
            console.error(err);
            document.querySelector("#results").textContent = "Error calling API";
        }
    } else {
        document.querySelector("#results").textContent = "Please enter your password.";
    }
});

const toggleButton = document.querySelector("#theme-toggle");
const savedTheme = localStorage.getItem("theme");

if (savedTheme === "dark") {
    document.body.classList.add("dark");
}

toggleButton.addEventListener("click", () => {
    document.body.classList.toggle("dark");

    if (document.body.classList.contains("dark")) {
        localStorage.setItem("theme", "dark");
    } else {
        localStorage.setItem("theme", "light");
    }
});