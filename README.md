# ToDoList

A simple command-line ToDo application written in Rust, designed to manage tasks without the need for a database.

## Features

- **Add Tasks**: Easily add new tasks to your list.
- **View Tasks**: Display all your tasks with their statuses.
- **Complete Tasks**: Mark tasks as completed.
- **Delete Tasks**: Remove tasks from your list.

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/boring-personality/ToDoList.git
    cd ToDoList
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

3. **Run the application**:
    ```sh
    ./target/release/todo
    ```
## Usage
- **Add a new task:**

    ```bash
    ./todo add "Your new task"
    ```

- **List all tasks:**

    ```bash
    ./todo list
    ```

- **Mark a task as completed:**

    ```bash
    ./todo complete <task_id>
    ```

- **Delete a task:**

    ```bash
    ./todo delete <task_id>
    ```
- **Clear all tasks:**

    ```bash
    ./todo clear
    ```
- **Open application in interactive mode:**

   ```bash
   ./todo interactive
   ```
## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

