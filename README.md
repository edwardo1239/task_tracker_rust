# Task Tracker CLI

A simple command-line interface (CLI) task tracking application written in Rust. This project is inspired by and follows the guidelines from the [Task Tracker project on roadmap.sh](https://roadmap.sh/projects/task-tracker).

## About This Project

This Task Tracker CLI is part of the practical projects series from [roadmap.sh](https://roadmap.sh), a platform that provides learning paths and project ideas for developers. The Task Tracker project is designed to help developers practice building a command-line application while learning about file I/O, data structures, and basic CRUD (Create, Read, Update, Delete) operations.

## Features

- Add new tasks
- Update existing tasks
- Delete tasks
- Mark tasks as in-progress
- Mark tasks as done
- List all tasks, completed tasks, or in-progress tasks

## Prerequisites

- Rust programming language (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/task-tracker-rust.git
   cd task-tracker-rust
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage

Once you run the application, you'll be presented with a prompt: `task-cli`

Here are the available commands:

- `add <task description>`: Add a new task
- `update <task_id> <new_description>`: Update an existing task
- `delete <task_id>`: Delete a task
- `mark-in-progress <task_id>`: Mark a task as in-progress
- `mark-done <task_id>`: Mark a task as done
- `list <option>`: List tasks (options: all, done, in-progress)

### Examples

```
task-cli add Buy groceries
task-cli update 1 Buy groceries and cleaning supplies
task-cli mark-in-progress 1
task-cli list all
task-cli mark-done 1
task-cli delete 1
```

## Project Structure

- `src/main.rs`: The entry point of the application
- `src/models/`: Contains the data models (Config, Tarea)
- `src/persistence/`: Handles task storage and operations
- `src/utils/`: Utility functions for file and I/O operations

## Dependencies

This project uses the following dependencies:

- `serde_json`: For JSON serialization and deserialization
- `std`: Rust standard library

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [roadmap.sh](https://roadmap.sh) for providing the project idea and guidelines
- The Rust community for their excellent documentation and resources

