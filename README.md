# tzconv

A lightweight desktop application for converting between Unix timestamps and human-readable datetimes across any timezone.

## Features

- Convert a Unix timestamp to a local datetime in any timezone
- Convert a local datetime (`yyyy-mm-dd HH:MM:SS`) back to a Unix timestamp
- Fill both fields with the current time at the click of a button
- Copy either value to the clipboard instantly
- Search and select from all IANA timezone identifiers via a combo box
- Handles DST gaps and ambiguous times gracefully

## Requirements

- [Rust](https://www.rust-lang.org/) 1.85 or later (edition 2024)

## Build

```bash
cargo build --release
```

The binary will be placed at `target/release/tzconv`.

## Usage

Run the application:

```bash
cargo run --release
```

| Field | Description |
|-------|-------------|
| **Unix** | Unix timestamp (seconds since the epoch) |
| **Date** | Local datetime in `yyyy-mm-dd HH:MM:SS` format |
| **TZ**   | IANA timezone identifier (e.g. `Asia/Tokyo`, `America/New_York`) |

### Workflow

1. Select a timezone from the **TZ** combo box (type to filter).
2. **Unix → Datetime** — Enter a Unix timestamp in the *Unix* field and press Enter (or click anywhere that submits the form).
3. **Datetime → Unix** — Enter a local datetime in the *Date* field and press Enter.
4. Click **Now** to populate both fields with the current time.
5. Click **Copy** next to either field to copy its value to the clipboard.

## License

This project is licensed under the [MIT License](LICENSE).
