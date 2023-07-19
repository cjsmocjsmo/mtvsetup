FROM rust:bookworm AS builder

RUN mkdir /usr/src/mtvsetup
WORKDIR /usr/src/mtvsetup
COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir /usr/src/mtvsetup/src
WORKDIR /usr/src/mtvsetup/src
COPY setup .
COPY main.rs .
COPY servermovie.rs .
COPY servertvs.rs .


RUN cargo install --path .

# FROM ubuntu:22.04
FROM debian:bookworm-slim

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get install -y sqlite3 build-essential && \
  apt-get -y autoremove && \
  apt-get -y autoclean

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup
RUN chmod +x /usr/local/bin/mtvsetup
WORKDIR /root/

RUN \
  mkdir ./usb1 && chmod -R +rwx ./usb1 && \
  mkdir ./usb2 && chmod -R +rwx ./usb2 && \
  mkdir ./usb3 && chmod -R +rwx ./usb3 && \
  mkdir ./usb4 && chmod -R +rwx ./usb4 

STOPSIGNAL SIGINT

# CMD ["tail", "-f", "/dev/null"]
CMD ["/usr/local/bin/mtvsetup"]
