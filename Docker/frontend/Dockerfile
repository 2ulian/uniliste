FROM node:20-alpine

# Enable pnpm via corepack
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy only dependency files to leverage cache
COPY ./frontend/package.json ./frontend/pnpm-lock.yaml* ./
RUN pnpm install

# Copy all frontend code
COPY ./frontend .

# Vite dev server (port defined in vite.config.js)
EXPOSE 1420
CMD ["pnpm", "dev", "--host", "--port", "1420"]
