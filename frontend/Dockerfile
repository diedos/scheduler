FROM node:18.12.1-slim AS development

WORKDIR /opt
COPY . ./

RUN npm install -g pnpm
RUN pnpm install

CMD [ "pnpm", "run", "dev" ]

FROM development AS builder

RUN pnpm run build
FROM nginx:1.24-alpine
COPY --from=builder /opt/public /usr/share/nginx/html