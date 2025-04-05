
# Person-DB

A lightweight Rust project for experimenting with web API development using popular crates.

## Prerequisites

- Docker installed and running
- Docker compose installed and running
- Basic command-line knowledge
- Rust toolchain (install via [rustup](https://rustup.rs/))

## Getting Started - Debugging locally with individual resources containers

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

The API will start on the default port (usually 8080).

## Getting started - Using docker compose file

### 1. Start Docker Containers

Run these commands to set up the required services:

```bash
# Navigate to the project directory containing docker-compose.yaml
cd path/to/person-db

# Start all services defined in docker-compose.yaml
docker-compose up -d
```

## Docker Configuration Files

### docker-compose.yaml

This file contains the configuration for running multiple services: PostgreSQL, Redis, RabbitMQ, and your Rust API. It's located in `src/`.

### Makefile

The Makefile is used to build and manage Docker images and containers. It includes targets for building a release version of your application or pushing an image to a Docker registry. Located in `src/`. You need to be logged in with your docker account and you need to change the path to your account, if you'd like to store your onw compiled image of "person-db" web api.

### nginx.conf

Nginx configuration file for load balancing between Rust API instances. Found in `src/`.

### definitions.json

RabbitMQ definition file to configure queues and permissions. Stored in `src/`.

### rabbitmq-init.sh

## Project Structure

```
.
├── src/                # Rust source code
├── db-init.sql         # Database schema
├── definitions.json    # RabbitMQ definitions
├── docker-compose.yaml # Docker compose file to run all application and dependencies locally
├── Dockerfile          # Creates the project image
├── MakeFile            # Helps create the project image and push to the docker hub
├── nginx.conf          # nginx configuration file containing load balancing rules
├── postgresql.conf     # Postgresql configuration file for development environment
├── rabbitmq.conf       # RabbitMQ configuration file for development environment
├── Cargo.lock          # Rust dependency lock file
├── Cargo.toml          # Rust dependencies
└── .env                # Environment variables for local and docker compose configuration
```

## Environment Variable needed:

| Variable Name                  | Description                                                  | Default Value          | Required |
|--------------------------------|--------------------------------------------------------------|------------------------|----------|
| `POSTGRES__HOST`               | Hostname for PostgreSQL database                             | `localhost`            | Yes      |
| `POSTGRES__PORT`               | Port for connecting to PostgreSQL                           | `5432`                 | Yes      |
| `POSTGRES__DB_NAME`            | Name of the PostgreSQL database                              | `person_db`            | Yes      |
| `POSTGRES__USER`               | Username for PostgreSQL authentication                      | `postgres`             | Yes      |
| `POSTGRES__PASSWORD`           | Password for PostgreSQL user                                 | `postgres`             | Yes      |
| `POSTGRES__POOL_SIZE`          | Size of the connection pool for PostgreSQL                   | `30`                   | No       |
| `POSTGRES__ACQUIRE_TIMEOUT`    | Maximum time to acquire a connection from the pool           | `30`                   | No       |
| `POSTGRES__IDLE_TIMEOUT`       | Maximum time a connection can remain idle in the pool        | `60`                   | No       |
| `REDIS__HOST`                  | Hostname for Redis server                                    | `localhost`            | Yes      |
| `REDIS__PORT`                  | Port for connecting to Redis                                 | `6379`                 | Yes      |
| `RABBIT__HOST`                 | Hostname for RabbitMQ server                                  | `localhost`            | Yes      |
| `RABBIT__PORT`                 | Port for connecting to RabbitMQ                               | `5672`                 | Yes      |
| `RABBIT__USER`                 | Username for RabbitMQ authentication                         | `guest`                | Yes      |
| `RABBIT__PASSWORD`             | Password for RabbitMQ user                                    | `guest`                | Yes      |
| `RABBIT__QUEUE_NAME`           | Name of the queue to be used in RabbitMQ                      | `people-queue`         | Yes      |
| `SERVER__HOST`                 | Hostname or IP address for the API server                     | `localhost`            | Yes      |
| `SERVER__PORT`                 | Port on which the API server will listen                     | `8080`                 | Yes      |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

## License

This project is licensed under the GLP-3.0 license. Read the license here: [GLP-3.0](LICENSE)