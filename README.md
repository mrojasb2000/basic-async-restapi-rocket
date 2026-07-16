# ROCKET Framework

## Environment variables

- ROCKET_*
- ROCKET_IDENT="Merpay"
- ROCKET_TLS={certs="abc.pem",key="def.pem"}
- ROCKET_ADDRESS: The IP address to bind to (e.g., 0.0.0.0 for production or cloud environments).
- ROCKET_PORT: The network port the server listens on (e.g., 8080).
- ROCKET_PROFILE: Configures the active profile (such as debug or release).
- ROCKET_LOG_LEVEL: Adjusts terminal output detail (critical, normal, debug, or off).
- ROCKET_SECRET_KEY: Sets the 256-bit key used to encrypt and sign private cookies.
- ROCKET_WORKERS: Determines the exact number of threads for parallel tasks.
- ROCKET_CLI_COLORS: Toggles terminal colors on or off (true or false).

## Using ROCKET.toml file

```sh
- address = "127.0.0.1"
- port = 8000
- workers = 16
- keep_alive = 5
- ident = "Rocket"
- log_level = "normal"
- temp_dir = "/tmp"
- cli_colors = true

### Generate secret key using openssl command.
### E.g. openssl rand -base64 32
- secret_key = "..."

[default.limits]
- form = "32KiB"
- data-form = "2MiB"
- file = "1MiB"
- string = "8KiB"
- bytes = "8KiB"
- json = "1MiB"
- msgpack = "1MiB"

[default.shutdown]
- ctrlc = true
- signals = ["term"]
- grace = 5
- mercy = 5
- force = true

[default.tls]
- certs = "/some/directory/cert-chain.pem
- key = "/some/directory/key.pem
```

## Running application

```sh
ROCKET_PORT=4000 cargo run
```
