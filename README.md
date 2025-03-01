# Vault

Vault is a secure password manager and authenticator application built with Rust and Axum. It provides a web interface for managing passwords and generating one-time passwords (OTPs) for two-factor authentication (2FA).

## Features

- Secure storage of passwords
- Two-factor authentication (2FA) with OTPs
- Account recovery
- Import/export of passwords
- Storage of recovery codes

## Getting Started

### Prerequisites

- Rust
- Just (justfile)
- PostgreSQL

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/lucalewin/vault.git
    cd vault
    ```

2. Set up the database:
    ```sh
    export DATABASE_URL=postgres://user:password@localhost/vault
    ```

3. Run the server:
    ```sh
    just run
    ```

4. Open your browser and navigate to `http://localhost:8000`.

## Roadmap

- Account recovery
- Import/export of passwords
- Authenticator (OTPs)
  - Storage of recovery codes

## License

This project is licensed under the MIT License.
