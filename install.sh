#!/bin/sh

echo "Building Rust app"
cargo build --release

echo "Copying binary to /usr/local/bin (requires sudo)"
sudo cp target/release/typst-templatr /usr/local/bin/

echo "Creating symlink 'tt' to 'typst-templatr' in /usr/local/bin"
sudo ln -sf /usr/local/bin/typst-templatr /usr/local/bin/tt

echo "Installation complete. You can now run 'typst-templatr' or 'tt' from anywhere."

