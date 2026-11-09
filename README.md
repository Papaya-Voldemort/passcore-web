# Passcore Web

> Fast, transparent password scoring built for trust.

---

## Why This Exists

Passcore Web started from Passcore, a Rust password scoring library I built because I cared a lot about speed and optimization.

But not many people can use a crate, so I built Passcore Web so more people could use my tool!

At the same time, I had just started learning JavaScript and building frontend stuff, and I thought it would be fun to connect the two. I didn’t love how slow or bloated a lot of web tools feel, so I decided to keep all the heavy work in Rust and just build a simple frontend on top of it.

Also, a lot of password checkers online feel either:
- slow
- generic
- or slightly sketchy

So I wanted to build one that feels fast, clean, and actually trustworthy.

And I am a *little* into optimization so... you know where that went. 

---

## Core Idea

> Passcore Web helps users check password strength instantly without sacrificing trust.

---

## What Makes It Different

- Uses my own aggressive Rust based scoring engine, compiled to WebAssembly and run locally in your browser — not a generic checker
- Focuses heavily on speed — results are instant because no network round trip is involved
- Keeps things simple instead of trying to do too much
- Clean UI that doesn’t get in the way

It’s not trying to be the best at everything. It’s trying to do one thing really, *really* well.

---

## Live Demo

> [passcore.nelx.dev](https://passcore.nelx.dev)

---

## Technical Overview

### Stack

- Scoring engine: Rust (Passcore) compiled to WebAssembly, runs 100% client-side
- Backend: Rust + Axum (serves the static site + a lightweight stats/ping API)
- Frontend: HTML / CSS / JavaScript (ES module loading the `.wasm` directly)
- Protection: Rate limiting + backend safeguards

---

### Key Technical Decisions

- Rust was chosen because I wanted maximum performance and to learn a lower-level language — it now compiles to WebAssembly so scoring runs directly in the browser
- Your password is never sent to the server. The frontend scores it locally and only sends an anonymous ping (the numeric score, never the password) so the public "passwords scored" counter keeps working
- The API is stateless so nothing sensitive needs to be stored
- Speed was the main priority from the start

Because it’s open source, you can verify exactly what’s happening.

---

## Performance

- Scoring runs locally in WebAssembly, so results are effectively instant — you type, and the result is already there
- The only server call is a tiny anonymous ping after each score (a few bytes), used purely for the public counter

---

## User Experience Philosophy

Passcore Web is designed to feel:

- fast - no, like **REALLY** fast
- clear — no confusing output
- focused — one job, done well
- clean — no unnecessary UI clutter

This is probably the most polished UI I’ve ever built, and I wanted it to feel that way.

---

## Privacy & Trust

- Passwords are never sent to the server — scoring happens locally in your browser via WebAssembly
- The only thing sent to the server is an anonymous numeric score ping to keep the public counter up to date
- Passwords are never stored or logged
- Fully open source

You shouldn’t have to trust a black box with your password — now you don’t have to send it anywhere at all.

---

## How It Works

1. You enter a password
2. It is scored **locally in your browser** using Passcore compiled to WebAssembly
3. You instantly get a score, grade, and feedback
4. A lightweight anonymous ping (score only) is sent to the server so the public "passwords scored" counter stays up to date

That’s it. The `/score` API endpoint still exists for programmatic use, but the website itself never sends your password anywhere.

---

## Who This Is For

- Everyday users who want a quick, trustworthy password check
- Developers who care about performance and clean design

---

## Roadmap

- Improve feedback and scoring explanations
- Add more metrics and insights
- Move from global rate limiting to IP-based rate limiting

### Currently in Progress:

- Adding more metrics and stats (I love data)

Progress: Next ship/round of features currently focused on polish

---

## Contributing

Feel free to open issues for anything — bugs, ideas, or even just questions.

Pull requests are welcome too, and I’ll credit ANY contributors.

---

## License

MIT
