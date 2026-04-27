# Basketball Player Analysis Backend

A high-performance backend service built with **Rust**, focused on basketball player profile analysis and external system integration. This project was developed as a core learning milestone to master **System Thinking** and backend architecture.

## 🧠 Design Philosophy & System Thinking

This project is built on the foundation of resilient and modular design, emphasizing how different parts of a system interact:

* **Resilient Database Setup:** The system is designed to handle its own initialization. It will "continue to operate even if the database file is missing" by automatically creating the `.db` file and running migrations to build tables on the first launch.
* **Method-Specific Responsibility:** HTTP methods are strictly separated based on their duties. `GET` is for data retrieval only, while `POST` and `PUT` are used for state changes (Create/Update), ensuring a clean and predictable API.
* **Middleware as a Security Guard:** The authentication layer acts as a gatekeeper. It verifies the API token and, once cleared, "assigns the request to the handler to perform its next duty," demonstrating a clear understanding of the request-response pipeline.
* **Asynchronous Flow:** Leveraging `Tokio`, the system manages multiple tasks—like saving to a database and notifying via LINE—without blocking the main execution thread.

## 🚀 Features

* **Player Analysis:** Automated feedback based on shooting styles (e.g., One-motion vs. Others).
* **Secure CRUD:** Full management of player profiles (Create, Read, Update, Delete).
* **LINE Integration:** Real-time push notifications using the LINE Messaging API.
* **Compile-time Safety:** Uses `SQLx` to verify SQL queries against the actual database schema during compilation.
* **Observability:** Integrated `Tracing` for detailed system logging and debugging.

## 🛠 Tech Stack

| Category | Technology |
| :--- | :--- |
| **Language** | Rust (Focusing on Ownership, Borrowing, and Error Handling) |
| **Web Framework** | Axum |
| **Async Runtime** | Tokio |
| **Database** | SQLite with SQLx |
| **Notification** | LINE Messaging API |

## 📡 API Endpoints

| Method | Endpoint | Description | Auth Required |
| :--- | :--- | :--- | :--- |
| `POST` | `/analyze` | Create and analyze a new player profile | No |
| `GET` | `/history` | Retrieve all analyzed players | No |
| `GET` | `/history/{id}` | Get a specific player by ID | No |
| `PUT` | `/update/{id}` | Update player shooting style | **Yes (Bearer Token)** |
| `DELETE` | `/delete/{id}` | Remove a player profile | **Yes (Bearer Token)** |

---
**Developed by [Vesper.rs](https://github.com/nattapong18-en)** *Building robust systems with Rust.*
