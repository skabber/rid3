# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.238.1/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version (use ubuntu-22.04 or ubuntu-18.04 on local arm64/Apple Silicon): ubuntu-22.04, ubuntu-20.04, ubuntu-18.04
ARG VARIANT="jammy"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

# [Optional] Uncomment this section to install additional OS packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends build-essential curl

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=="/root/.cargo/bin:${PATH}"

RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown
RUN $HOME/.cargo/bin/rustup target add wasm32-wasi
# RUN curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
# RUN mv ./spin $HOME/.local/bin/spin
RUN $HOME/.cargo/bin/cargo install trunk
