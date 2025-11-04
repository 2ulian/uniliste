FROM node:20-alpine

# Enable pnpm via corepack
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy only dependency files to leverage cache
COPY ./uniliste/package.json ./uniliste/pnpm-lock.yaml* ./
RUN pnpm install

# Copy all frontend code
COPY ./uniliste .

# Vite dev server (port defined in vite.config.js)
EXPOSE 1420
CMD ["pnpm", "dev", "--host", "--port", "1420"]
