# Scheduler

Scheduler is a work in progress over-engineered web-based todo application. It is supposed to be a simple personal assistant that prioritizes mundane tasks and tells the user what to do next. This project also works as a personal playground to try out new things and maybe learn something in the process.

Pretty much everything it currently does could be achieved purely in the Svelte frontend, or even a basic spreadsheet, but where's the fun in that?

## Current tech stack

- Database: PostgreSQL
- REST API: Rust (+Axum, SQLx)
- Frontend: TypeScript, Svelte, Tailwind
- E2E testing: Playwright (TBD)

## Roadmap

- automatic task prioritization based on deadlines, estimated workloads and other criteria such as context tags
- context tags to group related tasks together
- pomodoro
- dark mode
- tests
- cloud deployment via Github Actions and Terraform
- OAuth login (Google?)
- repeating tasks
- hotkeys and attention to accessibility
- explore View Transitions API
- explore gRPC

### Stretch goals

- Telegram bot and/or Google Calendar integration
- some tinkering with D3.js (todo: what and why?)
- appointment scheduling (kinda like Calendly)

## Development

Rust backend can be started with `cargo watch -x run`. It will run on port 3000.

Frontend uses pnpm as the package manager. Svelte frontend can be started with `pnpm run dev`. It will run on port 5173.

TODO: Set up Docker containers for backend and frontend
