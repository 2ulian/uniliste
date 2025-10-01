FROM node:20-alpine

# Activer pnpm via corepack
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copier uniquement les fichiers de dépendances pour profiter du cache
COPY ./uniliste/package.json ./uniliste/pnpm-lock.yaml* ./
RUN pnpm install

# Copier tout le code du frontend
COPY ./uniliste .

# Vite dev server
EXPOSE 5173
CMD ["pnpm", "dev", "--host"]
