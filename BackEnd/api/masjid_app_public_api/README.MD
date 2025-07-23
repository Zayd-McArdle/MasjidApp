# MasjidApp Public API

This API is retrieving the events, prayer times, announcements, classes data for use by the frontend. This API is also
used for sending questions and donation data from users as well.

## Building the  API

Prerequisites

- Docker Desktop
- Rust Compiler (optional but)
- IDE/text-editor (e.g., RustRover, Intellij, Visual Studio Code, Neovim, etc)

Before you run the below command, please ensure you are in the api parent directory.
For example: `MasjidApp/BackEnd/api/`

To build the API run the below command

```    
docker build -t masjidapppublicapi -f masjid_app_public_api/Dockerfile .
```

