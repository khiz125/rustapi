# Rust API - Backend

## Project Overview

This project is a Rust backend API server designed with **Clean Architecture**.  
The main goal is to manage users/accounts and audio data with associated labels and notes.

**Current Scope: Domain Layer only**

---

## Architecture

The project follows **Clean Architecture** principles:

```text
interface  → HTTP handlers (future)
usecase    → Business logic (future)
domain     → Entities, Value Objects, Repository traits
infrastructure → Repository implementations (future)
```

- Domain

```
domain/
├ mod.rs
├ error.rs
├ user/
│ ├ mod.rs
│ ├ entity.rs
│ ├ repository.rs
│ └ vo/
│ ├ mod.rs
│ ├ user_id.rs
│ ├ user_name.rs
│ └ email.rs
├ audio/ # To be implemented
└ shared_vo/ # Common value objects (e.g., Email)
```

```

```
