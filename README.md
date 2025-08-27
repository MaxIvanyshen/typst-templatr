# typst-templatr

A command-line tool written in Rust to manage Typst templates. It allows you to initialize a configuration file, list available `.typ` templates, and add templates to your current directory via symbolic links.

## Features

- **Initialize Configuration**: Create a configuration file (`.typst-templatr.yaml`) to specify the path to your Typst templates directory.
- **List Templates**: Display all `.typ` files in the configured templates directory.
- **Add Templates**: Create a symbolic link to a `.typ` template file from the templates directory to the current working directory.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version recommended)
- A Unix-like system (Linux, macOS) or Windows for symbolic link support

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/typst-templatr.git
   cd typst-templatr
   ```

2. **Using the Install Script (Unix-like Systems)**:
   Run the provided `install.sh` script to build and install the `typst-templatr` binary to `/usr/local/bin` and create a shorthand symlink `tt`:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```
   Note: The script may require `sudo` for installing to `/usr/local/bin` and creating the `tt` symlink. After installation, you can use both `typst-templatr` and `tt` to run the tool.

3. **Manual Installation**:
   If you prefer to build and install manually or are on a non-Unix system:
   ```bash
   cargo build --release
   sudo cp target/release/typst-templatr /usr/local/bin/
   sudo ln -s /usr/local/bin/typst-templatr /usr/local/bin/tt
   ```
   Alternatively, use `cargo install` to install to your user’s Cargo bin directory (note: this does not create the `tt` symlink):
   ```bash
   cargo install --path .
   ```

4. Verify installation:
   ```bash
   typst-templatr --help
   ```
   or
   ```bash
   tt --help
   ```
   This should display the help message for the `typst-templatr` command.

## Usage

The tool supports three subcommands: `init`, `list`, and `add`. You can use either `typst-templatr` or `tt` as the command.

### Initialize Configuration

Create a configuration file (`.typst-templatr.yaml`) in your home directory to specify the templates directory.

```bash
typst-templatr init --templates_path ~/.typst-templates
```
or
```bash
tt init --templates_path ~/.typst-templates
```

- `--templates_path`: Path to the directory containing your `.typ` template files (defaults to `~/.typst-templates`).
- Creates a `.typst-templatr.yaml` file in your home directory.

### List Templates

List all `.typ` files in the configured templates directory.

```bash
typst-templatr list
```
or
```bash
tt list
```

- Outputs the names of all `.typ` files in the templates directory, one per line, prefixed with a dash (e.g., `- template1.typ`).

### Add a Template

Create a symbolic link to a `.typ` template file from the templates directory to the current working directory.

```bash
typst-templatr add template_name
```
or
```bash
tt add template_name
```

- `template_name`: The name of the template file (with or without the `.typ` extension).
- Creates a symlink in the current directory pointing to the template file in the templates directory.
- Example: `tt add mytemplate` links `~/.typst-templates/mytemplate.typ` to `./mytemplate.typ`.

### Example Workflow

1. Initialize the config:
   ```bash
   tt init --templates_path ~/my_templates
   ```

2. List available templates:
   ```bash
   tt list
   ```
   Output:
   ```
   - report.typ
   - article.typ
   ```

3. Add a template to the current directory:
   ```bash
   tt add report
   ```
   This creates a symlink `./report.typ` pointing to `~/my_templates/report.typ`.

## Configuration File

The configuration file (`.typst-templatr.yaml`) is stored in your home directory and contains the path to your templates directory. Example:

```yaml
templates_path: ~/.typst-templates
```

## Error Handling

- If the configuration file is missing, use `init` to create it.
- If the templates directory doesn't exist or is inaccessible, an error will be displayed.
- If a template file doesn't exist when using `add`, or if a file with the same name already exists in the current directory, an error will be shown.

## Dependencies

The project uses the following Rust crates:

- `clap`: For command-line argument parsing
- `directories`: For locating the user's home directory
- `serde` and `serde_yaml`: For reading/writing the YAML configuration file

See `Cargo.toml` for version details.

## Building and Running Locally

To run the project without installing:

```bash
cargo run --release -- <subcommand>
```

Example:
```bash
cargo run --release -- list
```

## Platform Support

- **Unix-like Systems (Linux, macOS)**: Uses `std::os::unix::fs::symlink` for creating symbolic links. The `install.sh` script simplifies installation and creates a `tt` symlink for convenience.
- **Windows**: Uses `std::os::windows::fs::symlink_file` or `symlink_dir` depending on the target. Note that symbolic link creation on Windows may require elevated permissions (e.g., running as Administrator) unless Developer Mode is enabled. The `install.sh` script is not supported on Windows.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
