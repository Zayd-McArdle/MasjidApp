# MasjidApp Backend

The backend for MasjidApp consists of two Axum APIs (written in Rust) and a data access layer (MySQL is main data source and Redis as a cache). The backend components are hosted in a docker container and are contained within a docker-compose.yml file.

## Hosting the Backend
Prerequisites

- Docker Desktop
- API & MySQL docker images need to be built
- IDE/text-editor (e.g., RustRover, Intellij, Visual Studio Code, Neovim, etc)

To host the backend, please ensure that you are in the BackEnd parent directory.
For example: `MasjidApp/BackEnd/`

To run the docker compose, execute the below command

```    
docker compose up
```
