# typst-templatr

A command-line tool written in Rust to manage Typst templates. It allows you to initialize a configuration file, list available `.typ` templates, and manage templates by adding, removing, installing, or uninstalling them via symbolic links or file operations.

## Features

- **Initialize Configuration**: Create a configuration file (`.typst-templatr.yaml`) to specify the path to your Typst templates directory.
- **List Templates**: Display all `.typ` files in the configured templates directory.
- **Add Templates**: Create a symbolic link to a `.typ` template file from the templates directory to the current working directory.
- **Remove Templates**: Remove a symbolic link to a `.typ` template from the current working directory.
- **Install Templates**: Copy a `.typ` template file to the configured templates directory.
- **Uninstall Templates**: Remove a `.typ` template file from the configured templates directory.

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

The tool supports six subcommands: `init`, `list`, `add`, `remove`, `install`, and `uninstall`. You can use either `typst-templatr` or `tt` as the command. Run `typst-templatr --help` or `tt --help` for an overview, or `<command> --help` for subcommand-specific help.

### Initialize Configuration

Create a configuration file (`.typst-templatr.yaml`) in your home directory to specify the templates directory.

```bash
typst-templatr init --templates_path ~/.typst-templates
```
or
```bash
tt init --templates_path ~/.typst-templates
```

- `--templates_path`: Path to the directory where Typst templates will be stored (defaults to `~/.typst-templates`).
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

- `template_name`: Name of the template file (with or without `.typ` extension).
- Creates a symlink in the current directory pointing to the template file in the templates directory.
- Example: `tt add mytemplate` links `~/.typst-templates/mytemplate.typ` to `./mytemplate.typ`.

### Remove a Template

Remove a symbolic link to a `.typ` template from the current working directory.

```bash
typst-templatr remove template_name
```
or
```bash
tt remove template_name
```

- `template_name`: Name of the template file to remove (with or without `.typ` extension).
- Removes the symlink from the current directory.
- Example: `tt remove mytemplate` removes `./mytemplate.typ`.

### Install a Template

Copy a `.typ` template file to the configured templates directory.

```bash
typst-templatr install path/to/template.typ
```
or
```bash
tt install path/to/template.typ
```

- `template_path`: Path to the `.typ` template file to install (with or without `.typ` extension).
- Copies the file to the templates directory specified in the config.
- Example: `tt install ./test.typ` copies `test.typ` to `~/.typst-templates/test.typ`.

### Uninstall a Template

Remove a `.typ` template file from the configured templates directory.

```bash
typst-templatr uninstall template_name
```
or
```bash
tt uninstall template_name
```

- `template_name`: Name of the template file to uninstall (with or without `.typ` extension).
- Removes the file from the templates directory.
- Example: `tt uninstall mytemplate` removes `~/.typst-templates/mytemplate.typ`.

### Example Workflow

1. Initialize the config:
   ```bash
   tt init --templates_path ~/my_templates
   ```

2. Install a template:
   ```bash
   tt install ./test.typ
   ```

3. List available templates:
   ```bash
   tt list
   ```
   Output:
   ```
   - test.typ
   - report.typ
   ```

4. Add a template to the current directory:
   ```bash
   tt add test
   ```

5. Remove the template symlink:
   ```bash
   tt remove test
   ```

6. Uninstall the template:
   ```bash
   tt uninstall test
   ```

## Configuration File

The configuration file (`.typst-templatr.yaml`) is stored in your home directory and contains the path to your templates directory. Example:

```yaml
templates_path: ~/.typst-templates
```

## Error Handling

- If the configuration file is missing, use `init` to create it.
- If the templates directory doesn't exist or is inaccessible, an error will be displayed.
- For `add`, if the template doesn't exist in the templates directory or a file with the same name exists in the current directory, an error will be shown.
- For `remove`, if the template symlink doesn't exist in the current directory, an error will be shown.
- For `install`, if the source file doesn't exist, an error will be shown.
- For `uninstall`, if the template doesn't exist in the templates directory, an error will be shown.

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
