FROM rust:bookworm AS builder

RUN mkdir /usr/src/mtvsetup
WORKDIR /usr/src/mtvsetup
COPY Cargo.toml .
COPY Cargo.lock .
COPY .env .

RUN mkdir /usr/src/mtvsetup/src
WORKDIR /usr/src/mtvsetup/src
COPY ./src/servertvs.rs .
COPY ./src/servermov.rs .
COPY ./src/main.rs .
COPY ./src/setup.rs .

RUN mkdir /usr/src/mtvsetup/src/setup
WORKDIR /usr/src/mtvsetup/src/setup
COPY ./src/setup .

WORKDIR /usr/src/mtvsetup

RUN cargo install --path .

# FROM ubuntu:22.04
FROM debian:bookworm-slim

ENV MTV_USB1="/root/mtvsetup/usb1"
ENV MTV_USB2="/root/mtvsetup/usb2"
ENV MTV_USB3="/root/mtvsetup/usb3"
ENV MTV_USB4="/root/mtvsetup/usb4"
ENV MTV_MOVIES_THUMBNAIL_PATH="/root/mtvsetup/thumbnails/"
ENV MTV_STATIC_PATH="/root/mtvsetup/static"
ENV MTV_RAW_ADDR="192.168.0.26"
ENV MTV_SERVER_ADDR="http://192.168.0.26"
ENV MTV_SERVER_PORT="8080"
ENV MTV_DOCKER_VAR="DOCKER"
ENV MTV_OFFSET="25"
ENV MTV_DB_PATH="/root/mtvsetup/mtv.db"

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get install -y sqlite3 && \
  apt-get -y autoremove && \
  apt-get -y autoclean

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup
RUN chmod +x /usr/local/bin/mtvsetup

RUN \
  mkdir /root/mtvsetup && \
  mkdir /root/mtvsetup/thumbnails

WORKDIR /root/mtvsetup

RUN touch ./mtv.db
COPY .env .
COPY db-schema.txt .

RUN \
  mkdir ./usb1 && chmod -R +rwx ./usb1 && \
  mkdir ./usb2 && chmod -R +rwx ./usb2 && \
  mkdir ./usb3 && chmod -R +rwx ./usb3 && \
  mkdir ./usb4 && chmod -R +rwx ./usb4

STOPSIGNAL SIGINT

CMD ["tail", "-f", "/dev/null"]
# CMD ["/usr/local/bin/mtvsetup"]
