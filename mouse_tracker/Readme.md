# Mouse Tracker

This is a simple Rust application that tracks the mouse cursor position and displays it in a window every second. The application uses the `native-windows-gui` and `winapi` libraries to create the graphical user interface and handle mouse events.

## Features

- Displays the current mouse cursor position in a GUI window.
- Updates the cursor position every second.
- Closes gracefully when the window is closed.

## Prerequisites

- Rust programming language: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- Docker (optional): If you want to run the application in an isolated container, you need Docker installed.

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/mouse_tracker.git
   cd mouse_tracker
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the executable:
   ```sh
   ./target/release/mouse_tracker
   ```

## Running with Docker

To run the application in an isolated Docker container, follow these steps:

1. Build the Docker image:
   ```sh
   docker build -t mouse_tracker .
   ```

2. Run the Docker container:
   ```sh
   docker run --rm mouse_tracker
   ```

## Dependencies

- `native-windows-gui`: A library for creating native Windows GUI applications.
- `native-windows-derive`: Procedural macros for `native-windows-gui`.
- `winapi`: Rust bindings to Windows API.

## File Structure

- `src/`: Contains the source code for the application.
    - `main.rs`: The main entry point for the application.
- `Cargo.toml`: Project configuration file, including dependencies.
- `Dockerfile`: Docker configuration file for building the Docker image.

## How It Works

1. The application initializes the NWG library and creates the main window.
2. A label is used to display the cursor position.
3. An `AnimationTimer` is set up to trigger every second.
4. On each timer tick, the application fetches the current cursor position using the Windows API and updates the label.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Acknowledgements

- [native-windows-gui](https://github.com/gabdube/native-windows-gui)
- [winapi](https://github.com/retep998/winapi-rs)

## Author

Sekator