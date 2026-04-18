# ModSync-Server

## Configuration

**.env**
```env
PORT=3456                       # API listening port
MODS_DIR=./mods                 # Primary mod folder
DEPS_DIR=./deps                 # Add any client-only deps here
SECRET=YOUR_REALLY_SECRET_KEY   # It acts as an API key
```

The `.env` must exist in the root directory of the server executable, or it must be registered in the environment variables.

## Build

```bash
git clone https://github.com/haruyq/ModSync-Server.git

cd ModSync-Server

cargo build
```