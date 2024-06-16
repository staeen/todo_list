# To-Do List Application Manual

## Overview

The To-Do List application is a command-line tool designed to help you manage your tasks efficiently directly from your terminal. With this application, you can add, list, remove, and mark tasks as complete.

## Installation

Ensure you have Rust installed on your system. If not, you can install it by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

## Getting Started

1. Clone the project repository from [GitHub](https://github.com/your_username/todo_list) or download the source code as a ZIP file and extract it to a directory of your choice.

2. Navigate to the root directory of the project in your terminal.

3. Run the command `cargo build` to compile the project.

## Usage

The To-Do List application supports the following commands:

### 1. Add a Task

Use the `add` command to add a new task to your to-do list. Syntax:

`cargo run add "<task_description>"`

Replace `<task_description>` with the description of the task you want to add. For example:

`cargo run add "Make a todo application"`


This will add a new task with the description "Make a todo application" to your to-do list.

### 2. List Tasks

Use the `list` command to display all the tasks in your to-do list. Syntax:

`cargo run list`


This command will list all the tasks along with their IDs and completion status.

### 3. Remove a Task

Use the `remove` command to remove a task from your to-do list. Syntax:

`cargo run remove <task_id>`


Replace `<task_id>` with the ID of the task you want to remove. For example:

`cargo run remove 1`


This will remove the task with ID 1 from your to-do list.

### 4. Mark a Task as Complete

Use the `complete` command to mark a task as complete. Syntax:

`cargo run complete <task_id>`


This will mark the task with ID 1 as complete.

## Examples

Here are some examples demonstrating how to use the To-Do List application:

1. **Add a task:**

   ```sh
   cargo run add "Make a todo application"

2. **List all tasks:**

   ```sh
   cargo run list
3. **Remove a task:**

   ```sh
   ccargo run remove 1

1. **Mark a task as complete:**

   ```sh
   cargo run complete 1

## Notes

- The application automatically saves your tasks to a file named tasks.json in the project directory. You don't need to worry about losing your tasks between sessions.
- If the tasks.json file does not exist, the application will create it automatically.
