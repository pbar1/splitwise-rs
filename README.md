# Splitwise SDK for Rust

## Client Implementation Notes

### Authentication

TODO

### Users

GET `/get_current_user`

- 200
  ```json
  {
    "id": 0,
    "first_name": "Ada",
    "last_name": "Lovelace",
    "email": "ada@example.com",
    "registration_status": "confirmed",
    "picture": {
      "small": "string",
      "medium": "string",
      "large": "string"
    },
    "notifications_read": "2017-06-02T20:21:57Z",
    "notifications_count": 12,
    "notifications": {
      "added_as_friend": true
    },
    "default_currency": "USD",
    "locale": "en"
  }
  ```
- 401
  ```json
  {
    "error": "Invalid API request: you are not logged in"
  }
  ```

GET `/get_user/{id}`

- 200 OK
  ```
  see /get_current_user
  ```
- 401 Invalid API key or OAuth access token
  ```
  see /get_current_user
  ```
- 403 Forbidden
  ```json
  {
    "errors": {
      "base": [
        "Invalid API request: you do not have permission to perform that action"
      ]
    }
  }
  ```
- 404 Not Found
  ```json
  {
    "errors": {
      "base": [
        "Invalid API Request: record not found"
      ]
    }
  }
  ```

POST `/update_user/{id}`

### Groups

### Friends

### Expenses

### Comments

### Notifications

### Other
