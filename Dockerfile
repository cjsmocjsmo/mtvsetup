FROM rust:buster AS builder

RUN mkdir /usr/src/mtvsetup
RUN mkdir /usr/src/mtvsetup/src
RUN mkdir /usr/src/mtvsetup/target

WORKDIR /usr/src/mtvsetup

COPY Cargo.toml .

COPY src/config.yaml ./src
COPY src/main.rs ./src
COPY src/mtv_clean.rs ./src
COPY src/mtv_env_vars.rs ./src
COPY src/mtv_image.rs ./src
COPY src/mtv_misc.rs ./src
COPY src/mtv_mp3_info.rs ./src
COPY src/mtv_process_mp3s.rs ./src
COPY src/mtv_process_music_images.rs ./src
COPY src/mtv_split.rs ./src
COPY src/mtv_walk_dirs.rs ./src



RUN cargo install --path .

# FROM arm32v6/alpine:latest
# FROM alpine:latest
FROM ubuntu:22.04

# RUN apk --no-cache add ca-certificates
WORKDIR /root/

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup
RUN \
  mkdir ./static && \
  chmod -R +rwx ./static && \
  mkdir ./fsData && \
  chmod -R +rwx ./fsData
  # mkdir ./logs && \
  # chmod -R +rwx ./logs && \
  # echo "Creating log file" > ./logs/mtvSetup.log && \
  # echo "Creating log file" > ./logs/mtvTV.log && \
  # echo "Creating log file" > ./logs/mtvMOV.log && \
  # chmod -R +rwx ./logs/mtvSetup.log && \
  # chmod -R +rwx ./logs/mtvTV.log && \
  # chmod -R +rwx ./logs/mtvMOV.log

STOPSIGNAL SIGINT
CMD ["mtvsetup"]
