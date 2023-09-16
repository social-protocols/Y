# <img src="logo.svg" width="24" /> Y

_(Early stage project)_

A social network with collective bayesian reasoning.

## Development

```bash
just reset-db
just develop
```

Open in browser: <https://localhost:3000>

## Resources

### Backend

- Programming language: [Rust](https://www.rust-lang.org/)
- HTTP Server: [axum](https://github.com/tokio-rs/axum)
- Template language: [maud](https://maud.lambda.xyz/)
- Error handling: [anyhow](https://docs.rs/anyhow/latest/anyhow/)
- Database Connection: [sqlx](https://github.com/launchbadge/sqlx)

### Frontend

- communication with the backend [Htmx](https://htmx.org/)
- state management [Alpine.js](https://alpinejs.dev/)

## Benchmarking

Start release web server:

```bash
cargo run --release
```

Then benchmark:

```bash
just benchmark
```
