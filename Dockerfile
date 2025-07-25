FROM us-docker.pkg.dev/gemini-code-dev/gemini-cli/sandbox:0.1.13

# root でパッケージをインストール（Rust の依存や curl など）
USER root

RUN apt-get update && apt-get install -y \
    build-essential \
    curl

# node ユーザーに切り替え
USER node

# 環境変数（Rust/Cargo のパス）
ENV CARGO_HOME=/home/node/.cargo
ENV RUSTUP_HOME=/home/node/.rustup
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# rustup を node ユーザーのホームにインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && echo 'export PATH=$CARGO_HOME/bin:$PATH' >> ~/.bashrc \
    && cargo --version

# 作業ディレクトリ
WORKDIR /app

# ソースコードをコピー（必要に応じて除外）
COPY . .

# デフォルトで bash を起動
CMD ["bash"]
