# MP3 ID3 Tag Editor

This project is a web-based MP3 ID3 tag editor built with Rust and WebAssembly. It allows users to upload MP3 files, view and edit their ID3 tags, and save the changes.

## Technologies Used

- Rust
- WebAssembly (Wasm)
- Yew (Rust framework for creating web applications)
- id3 (Rust library for reading and writing ID3 tags)
- Bulma (CSS framework for styling)

## Features

- Upload MP3 files
- Display and edit ID3 tags (including title, artist, album, etc.)
- View and edit chapter information
- Display album art
- Play MP3 audio
- Save changes to ID3 tags

## Building and Running Locally

To build and run this project locally, follow these steps:

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Install `trunk` by running:
   ```
   cargo install trunk
   ```

3. Clone the repository:
   ```
   git clone https://github.com/skabber/rid3.git
   cd rid3
   ```

4. Build the project:
   ```
   trunk build --release
   ```

5. To run the project locally for development, use:
   ```
   trunk serve
   ```

6. Open your web browser and navigate to `http://localhost:8080` to use the application.

## GitHub Actions and Deployment

This project uses GitHub Actions for continuous integration and deployment to GitHub Pages. The workflow is defined in `.github/workflows/build-pages.yaml`.

The workflow does the following:

1. Triggers on pushes to the `main` branch.
2. Sets up the Rust toolchain with the `wasm32-unknown-unknown` target.
3. Installs `trunk`.
4. Builds the project using `trunk build --release`.
5. Deploys the built files to GitHub Pages.

To enable GitHub Pages deployment:

1. Go to your repository's Settings.
2. Navigate to the "Pages" section.
3. Under "Source", select the branch where your built files are pushed (usually `gh-pages`).
4. Save the changes.

Your project will now be accessible at `https://your-username.github.io/rid3/`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Contact

If you have any questions or feedback, please open an issue on the GitHub repository.
