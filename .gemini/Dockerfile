FROM us-docker.pkg.dev/gemini-code-dev/gemini-cli/sandbox:0.25.2


# root でパッケージをインストール（Rust の依存や curl など）
USER root

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    python3-pip \
    python3-venv

# node ユーザーに切り替え
USER node

# uv のインストール
RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# 環境変数（Rust/Cargo/uv のパスとキャッシュ）
ENV CARGO_HOME=/home/node/.cargo
ENV RUSTUP_HOME=/home/node/.rustup
ENV UV_CACHE_DIR=/home/node/.cache/uv
ENV PATH="/home/node/.local/bin:${CARGO_HOME}/bin:${PATH}"

# rustup を node ユーザーのホームにインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.90.0 \
    && rustup component add llvm-tools-preview \
    && cargo install cargo-llvm-cov \
    && echo 'export PATH=$CARGO_HOME/bin:$PATH' >> ~/.bashrc \
    && cargo --version

# 作業ディレクトリ
WORKDIR /app

# デフォルトで bash を起動
CMD ["bash"]
