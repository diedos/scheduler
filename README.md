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
- pomodoro with automatic breaks
- dark mode
- tests
- cloud deployment via Github Actions and Terraform
- OAuth login (Google?)
- repeating tasks
- hotkeys and attention to accessibility
- API documentation (Swagger or similar)
- explore View Transitions API
- explore gRPC

### Stretch goals

- Telegram bot and/or Google Calendar integration
- some tinkering with D3.js (todo: what and why?)
- appointment scheduling (kinda like Calendly)

## Development

Run `docker compose up -d` and you should be good to go. Frontend will be served at `localhost:5173`, backend at `localhost:3000`.
