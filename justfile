run: vue-build
    cargo run

build-prod: vue-build
    cargo build --release

vue-build:
    cd frontend && npm run build
    mkdir -p static
    cp -r frontend/dist/* static/

vue-dev:
    cd frontend && npm run dev -- --host "0.0.0.0"


install: build-prod
    # if the service is already running, stop it
    systemctl stop vault || true
    # remove the files
    rm -f "/etc/systemd/system/vault.service"  # service
    rm -f "/opt/vault/server"                  # binary
    rm -rf "/opt/vault/static"
    # remove the old .env file
    rm -f "/opt/vault/.env"

    # create the new service
    cp -r "static/" "/opt/vault/static"
    cp ".env" "/opt/vault/.env"
    cp "target/release/vault" "/opt/vault/server"
    cp "vault.service" "/etc/systemd/system/vault.service"
    systemctl enable vault
    systemctl start vault
