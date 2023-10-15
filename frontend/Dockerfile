FROM node:18.12.1-slim AS base

WORKDIR /opt
COPY . ./

RUN npm install -g pnpm
RUN pnpm install

FROM base as development

EXPOSE 5173
CMD [ "pnpm", "run", "dev" ]

FROM base AS builder

RUN pnpm run build
FROM nginx:1.24-alpine
COPY --from=builder /opt/public /usr/share/nginx/html