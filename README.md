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

- Uses my own aggressive Rust based scoring engine, not a generic checker
- Focuses heavily on speed — results come back almost instantly
- Keeps things simple instead of trying to do too much
- Clean UI that doesn’t get in the way

It’s not trying to be the best at everything. It’s trying to do one thing really, *really* well.

---

## Live Demo

https://passcore-web-production.up.railway.app/

If the API fails it's because too many people are using it at a time (will fix soon!)

---

## Technical Overview

### Stack

- Backend: Rust + Axum
- Frontend: HTML / CSS / JavaScript
- Architecture: Stateless API
- Protection: Rate limiting + backend safeguards

---

### Key Technical Decisions

- Rust was chosen because I wanted maximum performance and to learn a lower-level language
- The API is stateless so nothing sensitive needs to be stored
- Speed was the main priority from the start

One tradeoff is that using an API can feel less safe to some users, but nothing is stored or logged. Everything is processed and returned immediately.

Because it’s open source, you can verify exactly what’s happening.

---

## Performance

- Median response time: ~4ms
- p99 response time: ~48ms

In practice, this just feels instant. You type, and the result is basically already there.

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

- Passwords are never stored
- Inputs are only used to generate a score and feedback
- No tracking or logging of passwords
- Fully open source

You shouldn’t have to trust a black box with your password.

---

## How It Works

1. You enter a password
2. It gets sent to the API and scored using Passcore
3. You get a score, grade, and feedback

That’s it.

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