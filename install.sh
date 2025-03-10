#!/bin/bash

# Download the zip folder
curl -L -o release.zip https://github.com/lucalewin/vault/releases/latest/download/release.zip

# Unzip the folder
unzip release.zip -d release

# if the service is already running, stop it
systemctl stop vault || true
# remove the files
# rm -f "/etc/systemd/system/vault.service"  # service
rm -f "/opt/vault/server"                  # binary
rm -rf "/opt/vault/static"

# create the new service
cp -r "release/frontend/dist/" "/opt/vault/static"
cp "release/target/release/vault" "/opt/vault/server"
# cp "vault.service" "/etc/systemd/system/vault.service"
systemctl enable vault
systemctl start vault

# Clean up
rm release.zip
rm -r release