# OhMyGit - Self-Hosted Git Service

OhMyGit is a lightweight, self-hosted Git service designed for small to medium-sized development teams. Built as a full-stack web application using Rust, PostgreSQL, and Docker, it enables users to manage repositories, collaborate via a web interface, and perform Git operations such as push and clone. It provides a viable alternative to GitHub and GitLab without requiring large infrastructure or giving up control.

## Purpose
Run a self-hosted Git service with minimal hardware overhead, full transparency, and total control.

## Target Audience
Software engineers and system administrators looking for a self-hosted, low-dependency Git platform for small team collaboration.

##  Features
- Web-based interface to manage Git repositories
- Create repositories through the web UI
- Full Git push and clone support using the Smart HTTP protocol
- Role-based authorization

##  Design Decisions
- **Rust + [Axum](https://github.com/tokio-rs/axum) + [Diesel](https://diesel.rs/) + [Tokio](https://github.com/tokio-rs/tokio)**: For performance, safety, and async support
- **[git2](https://github.com/rust-lang/git2-rs)**: Libgit2 bindings for fine-grained Git control
- **[Askama](https://github.com/askama-rs/askama)**: Typed HTML templating for backend-rendered pages
- **PostgreSQL**: ACID-compliant and Docker-compatible
- **Argon2**: Secure password hashing
- **No frontend frameworks**: Minimize client resource usage
- **[Mockall](https://github.com/asomers/mockall)**: Simplify service-level unit testing

## Architecture Overview

### System Flow
```
Web Request:
[Browser] → [nginx] → [Frontend] → [Axum Handler] → [Service Layer] → [Diesel] → [PostgreSQL]

Git Clone:
[Terminal] → [nginx] → [Axum Handler] → [Service Layer] → [git2] → [Filesystem]
```

### Component Responsibilities
- **nginx**: Reverse proxy, TLS termination, request routing
- **Frontend**: Serves static HTML/JS/CSS and fallback pages
- **Backend (Rust/Axum)**: Handles routing, logic, and repository operations
- **Database (PostgreSQL)**: Stores user and repository metadata
- **git2**: Interfaces directly with Git repositories on disk

##  Backend Structure
- `api/`: Application startup, route mounting, request data extraction using handlers
- `application/`: Business logic, service layer, unit tests
- `domain/`: Core models and trait interfaces (User, Repository, Branch, Auth, Membership, ...)
- `infrastructure/`: Database access (Diesel), Git access (git2), filesystem operations, migrations
- `error/`: App-wide error management using `AppError`
- `state/`: `AppState` and service initialization
- `shared/`: Utilities for hashing, regex, and graceful shutdown
- `templating/`: HTML templates rendered via Askama

### Data Models
| Entity                | Fields                                                                       |
|----------------------|------------------------------------------------------------------------------|
| `User`               | `id`, `name`, `email`, `hashed_password`                                     |
| `Repository`         | `id`, `owner_id(FK)`, `owner_org_id(FK)`, `name`, `is_public`, `description` |
| `RepositoryBranch`   | `id`, `creator_id(FK)`, `repository_id(FK)`, `name`                          |
| `UserRepositoryRoles`| `user_id(FK)`, `repository_id(FK)`, `role`                                   |
The full database schema can be found in [schema](backend/infrastructure/migrations/2025-04-02-145354_initial_schema/up.sql).

## Frontend Overview
- Static login and signup pages in `static/`
- CSS/JS served from `/styles` and `/scripts`
- Uses JavaScript to fetch and POST data (e.g. repository and branch creation)
- Data structured to match backend deserialization models

## Important API Endpoints
| Method | Route                                           | Purpose                     |
|--------|--------------------------------------------------|-----------------------------|
| POST   | `/repos/create`                                 | Create a new repository     |
| POST   | `/login`, `/signup`                             | Authentication              |
| GET    | `/user/repo.git/info/refs`                      | Git discovery               |
| POST   | `/user/repo.git/git-receive-pack`               | Git push                    |
| POST   | `/user/repo.git/git-upload-pack`                | Git clone                   |

## Security
- TLS via nginx (Let's Encrypt)
- Backend session management (60s lifespan, in-memory only)
- Auth via email/username + password, protected via HTTPS
- Git auth uses Basic Auth (same credentials)
- Role-based auth enforced via database lookups
- Auto-generated nginx config mitigates XSS/CSRF

### Limitations
- No 2FA
- Sessions are stored on memory in the backend container

## Testing
- Unit tests for service logic using `cargo test`
- No integration tests yet

## Git Repository Management
- Bare repositories stored under `repos/username/repo.git`
- Mounted via Docker volume into backend
- Git Smart HTTP used for Git operations like clone and push

## Limitations & Future Roadmap
- Planned: Issues, Merge Requests, Comments, PostgreSQL Docker image size optimization, small TODO's in the codebase
- Not implemented: Web-based editing, CI/CD, Federation
- No graph database in place, so the current role-based model may not scale for teams > 50 users

## Setup Instructions

### Dev Setup
- make sure that port 80 is open on your system
```sh
# Install Docker
git clone https://github.com/schilling-p/ohmygit.git
cd ohmygit
docker-compose -f docker-compose-dev.yml up --build -d
```

### Manual Testing
```sh
# Open in browser
http://localhost

# Create account, repository, then:
git clone http://localhost/username/repo.git
git push origin main
```

### Production Setup
1. Rent a server with ports 80/443 open
2. Set DNS to point to your IP
3. Edit `docker-compose-prod.yml` (domain, email, database credentials)
4. Copy project files to server
5. Run `docker-compose -f docker-compose-prod.yml up --build -d`

### Update Strategy
```sh
git pull
docker-compose up --build -d
# Avoid using `-v` to preserve certificates 
```

## Contribution
This project is licensed under the MIT License. Contributions are welcome via merge requests at:

 [GitHub Repository](https://github.com/schilling-p/ohmygit#)