# <img src="logo.svg" width="24" /> Y

*(Early stage project)*

Enable useful discussions among thousands of people.

The idea is to create a discussion interface which resembles the mechanics of a real-world one-to-one discussion. But instead of having a single person as a counterpart, a user has a large crowd of people on the other end. By offering familiar actions, like *ask a yes-no question*, *answer a yes-no question*, *clarify definitions and/or context*, users can apply strategies and experience they know from real-world discussions. They don't have to learn a new paradignm to engage and contribute in a discussion.

Try it: <https://y.fly.dev>

## Development

```bash
just reset-db
just develop
```
Open in browser: <https://localhost:8000>

## Benchmarking

Start release web server:

```bash
cargo run --release
```

Then benchmark:

```bash
just benchmark
```
