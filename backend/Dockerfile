FROM rust:1.94-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_INSTALL_ROOT=/usr/local
RUN cargo install watchexec-cli

COPY . .

EXPOSE 3000

CMD ["watchexec", "-e", "rs,html,toml", "-r", "--", "cargo", "run"]
