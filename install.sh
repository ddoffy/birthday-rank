#!/bin/bash
set -e

APP_NAME="birthday-rank"
INSTALL_DIR="/opt/$APP_NAME"
SERVICE_FILE="/etc/systemd/system/$APP_NAME.service"

echo "Building $APP_NAME..."
cargo build --release

echo "Installing to $INSTALL_DIR..."
sudo mkdir -p $INSTALL_DIR
sudo cp target/release/$APP_NAME $INSTALL_DIR/
sudo cp birthday_ranks_2026.csv $INSTALL_DIR/

echo "Creating systemd service..."
cat <<EOF | sudo tee $SERVICE_FILE
[Unit]
Description=Birthday Rank Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/$APP_NAME
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

echo "Reloading systemd..."
sudo systemctl daemon-reload
sudo systemctl enable $APP_NAME
sudo systemctl start $APP_NAME

echo "Installation complete! Service is running on port 6464."
echo "Check status with: sudo systemctl status $APP_NAME"
