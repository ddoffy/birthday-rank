# Birthday Rank Service

A simple web service to check birthday ranks based on 2026 data.

## Prerequisites

- Rust (cargo)
- Systemd (Linux)

## Installation

1. Run the install script:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

2. The service will be installed to `/opt/birthday-rank` and started automatically.

## Usage

Open your browser and navigate to:
`http://<your-server-ip>:6464`

Select your birth month and day to see your rank.

## Development

To run locally:
```bash
cargo run
```
Then visit `http://localhost:6464`.
