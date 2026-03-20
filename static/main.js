// LoadStats is a function that calls the /stats portion of the API to find the current amount of passwords the API has scored
async function loadStats() {
    const statsText = document.querySelector("#passwords-scored");

    try {
        const res = await fetch(`/stats`);

        if (!res.ok) {
            throw new Error(`HTTP error! status: ${res.status}`);
        }

        const data = await res.json();

        statsText.textContent = data.total_scored;
    } catch (err) {
        console.error(err);
        statsText.textContent = "Error loading stats";
    }
}

// Initial load of stats on page start
loadStats();

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

const passwordInput = document.querySelector("#password");
const results = document.querySelector("#results");

let debounceTimer;
let controller;

// Main scoring logic on input
form.addEventListener("input", async (e) => { // TODO: Prevent enter from reloading the app
    const password = passwordInput.value.trim();
    clearTimeout(debounceTimer);

    if (controller) {
        controller.abort();
    }

    if (password === "") {
        results.textContent = "Please enter your password.";
        bar.style.width = "0%";
        return;
    }

    debounceTimer = setTimeout(async () => {
        controller = new AbortController();

        try {
            const res = await fetch("/score", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ password }),
                signal: controller.signal
            });

            if (!res.ok) {
                throw new Error(`HTTP ${res.status}`);
            }

            const data = await res.json();

            results.textContent = `Score: ${data.score} | Grade ${data.grade}`;
            bar.style.width = `${data.score}%`;
            bar.style.background = getGradeColor(data.grade) || "gray";
        } catch (err) {
            if (err.name !== "AbortError") {
                console.error(err);
                results.textContent = "Error calling API";
            }
        }
    }, 250);

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

const VisibilityToggle = document.querySelector("#visibility-toggle");
const PasswordInput = document.querySelector("#password");
const icon = document.querySelector("#visibility-toggle svg");

const show = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" fill="none"/><path d="M128,56C48,56,16,128,16,128s32,72,112,72,112-72,112-72S208,56,128,56Z" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/><circle cx="128" cy="128" r="40" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/></svg>`
const hide = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" fill="none"/><line x1="48" y1="40" x2="208" y2="216" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/><path d="M154.91,157.6a40,40,0,0,1-53.82-59.2" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/><path d="M135.53,88.71a40,40,0,0,1,32.3,35.53" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/><path d="M208.61,169.1C230.41,149.58,240,128,240,128S208,56,128,56a126,126,0,0,0-20.68,1.68" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/><path d="M74,68.6C33.23,89.24,16,128,16,128s32,72,112,72a118.05,118.05,0,0,0,54-12.6" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/></svg>`

// Show and hide password button
VisibilityToggle.addEventListener("click", () => {
    if (PasswordInput.type === "password") {
        PasswordInput.type = "text";
        icon.innerHTML = hide;
    } else {
        PasswordInput.type = "password";
        icon.innerHTML = show;
    }
});

// Prevent form from reloading on "enter"
form.addEventListener("submit", function(e) { e.preventDefault(); });