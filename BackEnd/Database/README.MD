# MasjidApp Database
The database used by MasjidApp is MySQL and is used as the main data source the MasjidApp API.

## Building the  API

Prerequisites

- Docker Desktop
- IDE/text-editor (e.g., RustRover, Intellij, Visual Studio Code, Neovim, etc)

Before you run the below command, please ensure you are in the Database parent directory.
For example: `MasjidApp/BackEnd/Database/`

To build the database run the below command

```    
docker build -t masjidappdatabase .
```

