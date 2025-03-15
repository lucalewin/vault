#!/bin/bash

# Download the zip folder
curl -L -o release.zip https://github.com/lucalewin/vault/releases/latest/download/release.zip

# Unzip the folder
unzip release.zip -d release

# if the service is already running, stop it
systemctl stop vault || true
rm -f "/opt/vault/server"
rm -rf "/opt/vault/static"

# create the new service
cp -r "release/static" "/opt/vault/static"
cp "release/vault" "/opt/vault/server"

# enable and start the service
systemctl enable vault
systemctl start vault

# Clean up
rm release.zip
rm -r release