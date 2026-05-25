FROM rust:1.94-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN ARCH=$(uname -m) && \
    wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-${ARCH}-unknown-linux-gnu.tar.gz \
    | tar -xzf- -C /usr/local/bin

COPY . .

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080"]
