# uniliste

Web app with Vue 3 frontend and Axum (Rust) backend.

## Development

Dockerized dev stack (Vite on 1420, Axum on 8000, Mongo on 27017):

```bash
docker compose up --build
```

Then open http://localhost:1420. Frontend calls backend through `/api` proxy to `http://backend:8000` (see `uniliste/vite.config.js`).

## Notes

- Tauri has been fully removed. The app now runs purely as a web app.