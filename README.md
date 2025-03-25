
# Person-DB

A lightweight Rust project for experimenting with web API development using popular crates.

## Prerequisites

- Docker installed and running
- Basic command-line knowledge
- Rust toolchain (install via [rustup](https://rustup.rs/))

## Getting Started

### 1. Start Docker Containers

Run these commands to set up the required services:

```bash
# PostGIS (PostgreSQL with GIS extensions)
docker run --name postgis \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  -v postgis_data:/var/lib/postgresql/data \
  --restart unless-stopped \
  -d postgis/postgis

# Redis
docker run --name redis \
  -p 6379:6379 \
  -v redis_data:/data \
  --restart unless-stopped \
  -d redis

# RabbitMQ (with management console)
docker run --name rabbitmq \
  -e RABBITMQ_DEFAULT_USER=guest \
  -e RABBITMQ_DEFAULT_PASS=guest \
  -p 5672:5672 \
  -p 15672:15672 \
  -v rabbitmq_data:/var/lib/rabbitmq \
  --restart unless-stopped \
  -d rabbitmq:4.0-management
```

### 2. Configure RabbitMQ

Access the management console:  
[http://localhost:15672](http://localhost:15672)

Login with:
- Username: `guest`
- Password: `guest`

Create a new queue named `people-queue`

### 3. Set Up PostgreSQL

1. Connect to PostgreSQL:
   ```bash
   psql -h localhost -U postgres
   ```
   (Password: `postgres`)

2. Create the database:
   ```sql
   CREATE DATABASE "person-db";
   ```

3. Initialize schema:
   ```bash
   psql -h localhost -U postgres -d person-db < db-init.sql
   ```

### 4. Run the Application

```bash
cargo run
```

The API will start on the default port (usually 8000).

## Project Structure

```
.
├── src/             # Rust source code
├── db-init.sql      # Database schema
├── Cargo.toml       # Rust dependencies
└── .env.example     # Environment variables template
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

## License

This project is licensed under the GLP-3.0 license. Read the license here: [GLP-3.0](LICENSE)