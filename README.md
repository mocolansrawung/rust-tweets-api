# Tweet Rocket SQLite App

## Description

This project is a simple Rust application using the Rocket web framework and SQLite database to manage tweets. It allows users to create, read, and retrieve tweets from the SQLite database.

## Features

- Create new tweets with a name and description.
- View a list of all tweets.
- View details of a specific tweet by its ID.

## Installation

1. **Clone the repository:**

   ```git clone https://github.com/your-username/tweet-rocket-sqlite-app.git```

2. **Navigate to the project directory:**

   ```cd tweet-rocket-sqlite-app```

3. **Build and run the project with Cargo:**

  ```cargo run```

## Usage

Once the project is running, you can interact with the API using HTTP requests. Here are some example requests:

- **Create a new tweet:**
  - Method: POST
  - URL: http://localhost:8000/tweets
  - Request body (JSON):
    ```json
    {
      "name": "mocolansrawung",
      "description": "rust keren bgt cuy!"
    }
    ```

- **Get a list of all tweets:**
  - Method: GET
  - URL: http://localhost:8000/tweets

- **Get details of a specific tweet:**
  - Method: GET
  - URL: http://localhost:8000/tweets/<id>
  - Replace `<id>` with the ID of the tweet you want to retrieve.

## Dependencies

- Rocket: Web framework for Rust
- SQLx: Async SQL query builder and executor for Rust
- SQLite: Embedded relational database engine

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
