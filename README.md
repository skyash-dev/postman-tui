# POSTMAN-TUI

![postman-tui](image.png)

**POSTMAN-TUI** is a terminal-based HTTP client built using Ratatui, inspired by Postman. It allows you to make HTTP requests and view responses directly in your terminal, providing a lightweight and efficient alternative for developers who prefer using the terminal.

## Features

- **GET Requests:** Currently supports sending GET requests.
- **TUI Interface:** Simple and interactive terminal user interface.
- **Lightweight:** Quick to launch and easy to use.
- **Open Source:** Contributions are welcome!

## Installation

To install **POSTMAN-TUI**, use `cargo` (the Rust package manager):

```bash
cargo install postman-tui
```

This will download and build the latest version of POSTMAN-TUI from crates.io.

## Usage

After installation, you can start using POSTMAN-TUI by running:

```bash
postman-tui
```

## Switching Between Tabs

You can switch between the following HTTP method tabs:

- GET
- POST
- PUT
- PATCH
- DELETE

To navigate between tabs, you can use:

- **Tab** or **'l'** to switch to the right tab.
- **Shift+Tab** or **'h'** to switch to the left tab.

## Editing the HTTP Verb

To change the HTTP verb (e.g., from GET to POST):

- Press 'v' to edit the verb.
- Choose the new verb (GET, POST, PUT, PATCH, DELETE).
- Press escape to get back to main screen.

## Editing the URL

To edit the URL:

- Press 'u'.
- Enter the new URL.
- Press Enter to send the request.

## Sending a Request

After setting the verb and URL, press Enter to send the request.
View the response headers and body in the terminal.

## Navigation

Use the arrow keys to navigate through response.
Press escape then q to quit the application.

### TODO

- [ ] Input Area For Requests. (Example: JSON, Body, Query, etc.)
- [ ] Other Requests Working. (POST, PUT, PATCH, DELETE)
