# Vault

Vault is a secure password manager and authenticator application written in Rust. It provides a web interface for managing passwords and generating one-time passwords (OTPs) for two-factor authentication (2FA).

## Features

- Secure storage of passwords
- Two-factor authentication (2FA) with OTPs
- Account recovery
- Import/export of passwords
- Storage of recovery codes

## Installation

```bash
curl -L https://raw.githubusercontent.com/lucalewin/vault/main/install.sh | sudo bash
```

Note: Currently this only works on `aarch64 linux gnu` (example: a Raspberry Pi 4)

## Roadmap

- [ ] Account recovery
- [ ] Import of passwords
- [x] Export of passwords
- [ ] Authenticator (OTPs)
  - [ ] Storage of recovery codes

## License

This project is licensed under the MIT License.
