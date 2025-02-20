<!-- # User Management
- **POST /users**: Create a new user account
  - Example: `POST /users`
  - Request Body: `{ "username": "user1", "password": "pass123", "email": "user1@example.com" }`
  - Response: `{ "userId": "123", "username": "user1", "email": "user1@example.com" }`

- **GET /users/{userId}**: Get user details
  - Example: `GET /users/123`
  - Response: `{ "userId": "123", "username": "user1", "email": "user1@example.com" }`

- **PUT /users/{userId}**: Update user details
  - Example: `PUT /users/123`
  - Request Body: `{ "email": "newemail@example.com" }`
  - Response: `{ "userId": "123", "username": "user1", "email": "newemail@example.com" }`

- **DELETE /users/{userId}**: Delete user account
  - Example: `DELETE /users/123`
  - Response: `{ "message": "User account deleted successfully" }`

- **POST /users/login**: Authenticate a user and obtain a token
  - Example: `POST /users/login`
  - Request Body: `{ "username": "user1", "password": "pass123" }`
  - Response: `{ "token": "jwt.token.here" }`

- **POST /users/logout**: Log out a user and invalidate the token
  - Example: `POST /users/logout`
  - Request Body: `{ "token": "jwt.token.here" }`
  - Response: `{ "message": "Logged out successfully" }`

- **POST /users/forgot-password**: Initiate password recovery process
  - Example: `POST /users/forgot-password`
  - Request Body: `{ "email": "user1@example.com" }`
  - Response: `{ "message": "Password recovery email sent" }`

- **POST /users/reset-password**: Reset the user password
  - Example: `POST /users/reset-password`
  - Request Body: `{ "token": "reset.token.here", "newPassword": "newpass123" }`
  - Response: `{ "message": "Password reset successfully" }` -->

# Credential Management
- **POST /credentials**: Create a new credential
  - Example: `POST /credentials`
  - Request Body: `{ "service": "example.com", "username": "user1", "password": "pass123" }`
  - Response: `{ "credentialId": "456", "service": "example.com", "username": "user1" }`

- **GET /credentials**: List all credentials for the authenticated user
  - Example: `GET /credentials`
  - Response: `[ { "credentialId": "456", "service": "example.com", "username": "user1" } ]`

- **GET /credentials/{credentialId}**: Get details of a specific credential
  - Example: `GET /credentials/456`
  - Response: `{ "credentialId": "456", "service": "example.com", "username": "user1", "password": "pass123" }`

- **PUT /credentials/{credentialId}**: Update a specific credential
  - Example: `PUT /credentials/456`
  - Request Body: `{ "password": "newpass123" }`
  - Response: `{ "credentialId": "456", "service": "example.com", "username": "user1", "password": "newpass123" }`

- **DELETE /credentials/{credentialId}**: Delete a specific credential
  - Example: `DELETE /credentials/456`
  - Response: `{ "message": "Credential deleted successfully" }`

# Two-Factor Authentication
- **POST /2fa/setup**: Set up two-factor authentication for the user
  - Example: `POST /2fa/setup`
  - Request Body: `{ "phoneNumber": "+1234567890" }`
  - Response: `{ "message": "2FA setup initiated", "qrCode": "data:image/png;base64,..." }`

- **POST /2fa/verify**: Verify the two-factor authentication code
  - Example: `POST /2fa/verify`
  - Request Body: `{ "code": "123456" }`
  - Response: `{ "message": "2FA verified successfully" }`

- **POST /2fa/disable**: Disable two-factor authentication for the user
  - Example: `POST /2fa/disable`
  - Request Body: `{ "code": "123456" }`
  - Response: `{ "message": "2FA disabled successfully" }`

<!-- # Encryption Management
- **POST /encryption/keys**: Rotate encryption keys
  - Example: `POST /encryption/keys`
  - Request Body: `{ "oldKey": "oldkeyhere", "newKey": "newkeyhere" }`
  - Response: `{ "message": "Encryption keys rotated successfully" }`

- **GET /encryption/status**: Get encryption status
  - Example: `GET /encryption/status`
  - Response: `{ "status": "Encryption is active", "lastRotated": "2025-01-01T00:00:00Z" }` -->