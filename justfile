run: dioxus-build
    cargo run

dioxus-build:
    cd frontend && dx build --release && tailwindcss -i ./input.css -o ./assets/tailwind.css
    mkdir static
    # cp -r frontend/dist/* static/