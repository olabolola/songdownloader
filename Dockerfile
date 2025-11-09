FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN mv /app/target/release/songdownloader /app/songdownloader
RUN chmod +x yt-dlp
RUN apt-get update
RUN apt install -y ffmpeg
CMD ["/app/songdownloader"]
