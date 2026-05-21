# coldvault

Secure, compressed, profile-based backups for modern infrastructure.

coldvault is a lightweight CLI for reliable backups with compression,
encryption, and deterministic execution.

Built for teams that want backups to be simple, reproducible, and secure.

## Features

- Profile-based backups
- Compression support (`zstd`)
- Encryption support (`age`, AES-256)
- Deterministic backup pipelines
- Config-driven execution
- Local and remote config support
- Extensible architecture

## Installation

### Cargo

```bash
cargo install coldvault
```

### Build from source

```bash
git clone https://github.com/<your-org>/coldvault.git
cd coldvault
cargo build --release
```

Binary:

```bash
./target/release/coldvault
```

## Quickstart

Initialize a config:

```bash
coldvault init
```

Run a backup:

```bash
coldvault run -p prod
```

Verify backup integrity:

```bash
coldvault verify
```

Restore a backup:

```bash
coldvault restore --file backup.zst.age
```

## Configuration

Example configuration:

```toml
[profiles.prod]
mongo_uri = "mongodb://localhost:27017"

output_dir = "./backups"

compression = "zstd"
encryption = "age"
```

Run using a profile:

```bash
coldvault run -p prod
```

## Profiles

Profiles allow environment-specific backup configurations.

Example:

```toml
[profiles.dev]
mongo_uri = "mongodb://localhost"

[profiles.prod]
mongo_uri = "mongodb://prod-db.internal"
```

Run:

```bash
coldvault run -p dev
coldvault run -p prod
```

## Documentation

## Philosophy

coldvault is designed around two principles:

1. Backups must be reliable
2. Running backups should require minimal effort

The goal is simple:

```bash
coldvault run -p prod
```

No fragile scripts. No repeated configuration.

## Roadmap

- [ ] MongoDB backups
- [ ] Compression pipeline
- [ ] Encryption pipeline
- [ ] Restore workflows
- [ ] Storage providers
- [ ] Remote config support
- [ ] Verification pipeline

## Contributing

Contributions, bug reports, and discussions are welcome.

Please open an issue before major changes.

## License

MIT License. See LICENSE file for details.
