
# Web Crawler in Rust

This project demonstrates how to build a simple web crawler and API requester in Rust. It uses the `reqwest` library for HTTP requests and the `scraper` library for extracting data from HTML. The bot can fetch data from websites, parse HTML, and make API requests to gather and display information.

## Features
- Send HTTP GET requests to websites and APIs
- Parse HTML content using `scraper` to extract specific elements
- Retrieve and display data from JSON APIs
- Store and process extracted data (optional for expansion)
- Supports asynchronous execution using `tokio`

## Prerequisites
Ensure you have the following installed on your system:
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **SQLite (optional)**: If you wish to store crawled data.
- **dotenv (optional)**: To manage environment variables (API keys, etc.).

## Installation and Setup

### 1. Clone the Repository
Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/web-crawler.git
cd web-crawler
```

### 2. Set up Dependencies
Ensure that your dependencies are installed:

```bash
cargo build
```

This will fetch and build the necessary libraries for the project.

### 3. Create a `.env` File (Optional)
Create a `.env` file in the root of your project and add any environment variables like API keys if needed:

```
API_URL=https://your-api-url.com
```

### 4. Running the Bot
To run the bot, use the following command:

```bash
cargo run
```

This will start the crawler, which will then fetch data from the specified website and API.

## Commands

This bot fetches data from the following:

- **Website**: The bot scrapes all the `h1` elements from the specified URL.
- **API**: The bot makes a GET request to a sample API (jsonplaceholder) and outputs the title and body of a post.

## Bot Structure

- **`main.rs`**: The main entry point of the bot. Handles web scraping, API requests, and data parsing.
- **`Cargo.toml`**: The file that defines dependencies and project configuration.
- **`.env` (optional)**: Stores environment variables for API configuration.

## Extending the Bot
You can extend this bot by:
- Adding more complex parsing for various HTML elements.
- Saving the extracted data into a database or file.
- Handling pagination to crawl multiple pages from a website.
- Making more complex API requests with authentication.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [reqwest](https://crates.io/crates/reqwest): Rust library for making HTTP requests.
- [scraper](https://crates.io/crates/scraper): Rust library for extracting data from HTML.
- [tokio](https://tokio.rs/): Asynchronous runtime used to handle requests concurrently.

