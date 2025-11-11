# auth_app (Rocket + SQLite + Tera)

Minimal authentication app built with:
- Rocket 0.5 (Rust web framework)
- SQLite via rusqlite
- Tera templates via rocket_dyn_templates
- bcrypt password hashing

## Features
- Register and login
- Cookie-based session with private cookies
- Request guard for protected routes
- Tera-rendered pages

## Project layout
```
src/
  main.rs                 # Rocket setup and mounts
  db.rs                   # SQLite init and user helpers
  routes/
    index.rs              # GET /
    auth.rs               # POST /login, GET/POST /register, GET /logout, AuthUser guard
    dashboard.rs          # GET /dashboard (protected)
templates/
  index.html.tera
  register.html.tera
  dashboard.html.tera
```

Key files:
- Mounts: src/main.rs
- DB helpers: src/db.rs (init_db, create_user, validate_user)
- Auth guard: src/routes/auth.rs (AuthUser)
- Routes: src/routes/index.rs, src/routes/auth.rs, src/routes/dashboard.rs

## Running
1. Ensure Rust stable is installed.
2. Run:
   ```
   cargo run
   ```
3. Open http://localhost:8000

First start creates auth.db and the users table.

## Routes
- GET / → Index with login form.
- POST /login → Authenticates, sets private cookie user_id.
- GET /register → Registration form.
- POST /register → Creates user (username unique).
- GET /dashboard → Requires session; redirects when unauthenticated.
- GET /logout → Clears session cookie.

## How it works
- Passwords are hashed with bcrypt before storage.
- Sessions use Rocket private cookies (encrypted + signed).
- The AuthUser request guard checks for the user_id cookie and gates access to the dashboard.

## Env/config
- To persist cookie encryption across restarts set a secret key:
  ```
  export ROCKET_SECRET_KEY=$(head -c 32 /dev/urandom | base64)
  cargo run
  ```
- Default port: 8000 (configure via ROCKET_PORT or Rocket.toml).

## Quick test (curl)
- Register:
  ```
  curl -i -X POST -d 'username=admin&password=password' http://localhost:8000/register
  ```
- Login and access dashboard:
  ```
  curl -i -c cookies.txt -X POST -d 'username=admin&password=password' http://localhost:8000/login
  curl -i -b cookies.txt http://localhost:8000/dashboard
  ```

## Cleaning
- Remove the local DB:
  ```
  rm -f auth.db
  ```

## License
Dual-licensed under MIT and Apache-2.0. See LICENSE-MIT and LICENSE-APACHE.