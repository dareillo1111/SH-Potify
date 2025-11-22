FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release


FROM ubuntu:24.04
RUN apt-get update && \
    apt-get install -y python3 python3-pip pipx ffmpeg && \
    pipx install yt-dlp && \
    pipx install spotdl

ENV PATH="/root/.local/bin:${PATH}"

WORKDIR /app

COPY --from=builder /app/target/release/sh_potify .

RUN mkdir -p /app/tracks
RUN mkdir -p /app/sqlite

VOLUME ["/app/sqlite"]
VOLUME ["/app/tracks"]

CMD ["./sh_potify"]

