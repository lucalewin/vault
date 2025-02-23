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
