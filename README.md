# auth_app (Rocket + SQLite + Tera)

Minimal authentication app built with:
- Rocket 0.5 (Rust web framework)
- SQLite via rusqlite
- Tera templates via rocket_dyn_templates
- bcrypt password hashing

## Features
- User registration and login
- Cookie-based session with private cookies (encrypted + signed)
- Request guard for protected routes
- Tera-rendered HTML templates
- Comprehensive integration tests

## Project Structure

```
auth_app/
├── src/
│   ├── main.rs                 # Binary entry point (re-exports lib)
│   ├── lib.rs                  # Library crate with Rocket setup
│   ├── db.rs                   # SQLite init and user helpers
│   └── routes/
│       ├── mod.rs              # Route module exports
│       ├── index.rs            # GET /
│       ├── auth.rs             # Login, register, logout, AuthUser guard
│       └── dashboard.rs        # GET /dashboard (protected)
├── templates/
│   ├── index.html.tera
│   ├── register.html.tera
│   └── dashboard.html.tera
└── tests/
    ├── routes_test.rs          # Integration tests
    └── common/
        └── mod.rs              # Test utilities
```

### Key Files
- **Entry point**: [src/lib.rs](src/lib.rs) - Rocket builder with all mounts
- **Binary**: [src/main.rs](src/main.rs) - Thin wrapper launching the app
- **Database**: [src/db.rs](src/db.rs) - Functions: `init_db`, `create_user`, `validate_user`
- **Auth guard**: [src/routes/auth.rs](src/routes/auth.rs) - `AuthUser` request guard
- **Routes**:
  - [src/routes/index.rs](src/routes/index.rs) - Home page
  - [src/routes/auth.rs](src/routes/auth.rs) - `login_submit`, `register_page`, `register_submit`, `logout`
  - [src/routes/dashboard.rs](src/routes/dashboard.rs) - `dashboard`, `dashboard_unauthenticated`
- **Tests**: [tests/routes_test.rs](tests/routes_test.rs) - Integration tests for all routes

## Running

1. Ensure Rust stable is installed.
2. Run:
   ```bash
   cargo run
   ```
3. Open http://localhost:8000

First start creates `auth.db` and the `users` table.

## Routes

| Method | Path         | Description                                      | Auth Required |
|--------|--------------|--------------------------------------------------|---------------|
| GET    | `/`          | Index page with login form                       | No            |
| POST   | `/login`     | Authenticates user, sets private cookie `user_id`| No            |
| GET    | `/register`  | Registration form                                | No            |
| POST   | `/register`  | Creates new user (username must be unique)       | No            |
| GET    | `/dashboard` | Protected dashboard (requires session)           | Yes           |
| GET    | `/logout`    | Clears session cookie                            | No            |

## How It Works

- **Passwords** are hashed with bcrypt before storage in [src/db.rs](src/db.rs).
- **Sessions** use Rocket private cookies (encrypted + signed via `CookieJar.add_private`).
- **Auth guard** [`AuthUser`](src/routes/auth.rs) checks for the `user_id` cookie and gates access to protected routes.
- **Protected route** [`dashboard`](src/routes/dashboard.rs) uses the `AuthUser` guard; unauthenticated users hit the fallback route [`dashboard_unauthenticated`](src/routes/dashboard.rs) (rank 2) which redirects to `/`.

## Testing

Run the full test suite:

```bash
cargo test
```

### Test Coverage

The project includes comprehensive integration tests in [tests/routes_test.rs](tests/routes_test.rs):

- **Index route** (`test_index`) - Verifies home page renders correctly
- **Register routes** (`test_register_page`, `test_register_new_user`, `test_register_duplicate_user`)
  - GET /register form
  - New user creation
  - Duplicate username handling
- **Login route** (`test_login_valid`, `test_login_invalid`)
  - Valid credentials
  - Invalid credentials
- **Dashboard routes** (`test_dashboard_authenticated`, `test_dashboard_unauthenticated`)
  - Authenticated access (with session cookie)
  - Unauthenticated redirect
- **Logout route** (`test_logout`) - Session cleanup

Tests use Rocket's blocking test client with automatic cookie management for session handling.

## Environment / Configuration

- **Secret key**: Rocket generates a random secret key in dev mode. To persist sessions across restarts, set:
  ```bash
  export ROCKET_SECRET_KEY=$(openssl rand -base64 32)
  cargo run
  ```
  Or add to `Rocket.toml`:
  ```toml
  [default]
  secret_key = "your-generated-key-here"
  ```
- **Port**: Default is `8000`. Configure via `ROCKET_PORT` or `Rocket.toml`:
  ```toml
  [default]
  port = 3000
  ```
- **Database**: SQLite file `auth.db` is created in the project root. Location can be changed in [src/db.rs](src/db.rs).

## Quick Test (curl)

Register a user:
```bash
curl -i -X POST -d 'username=admin&password=password' http://localhost:8000/register
```

Login and access dashboard (with cookies):
```bash
curl -i -c cookies.txt -X POST -d 'username=admin&password=password' http://localhost:8000/login
curl -i -b cookies.txt http://localhost:8000/dashboard
```

Logout:
```bash
curl -i -b cookies.txt http://localhost:8000/logout
```

## Cleaning

Remove the local database:
```bash
rm -f auth.db
```

Remove build artifacts:
```bash
cargo clean
```

## License

Dual-licensed under MIT and Apache-2.0.  
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
