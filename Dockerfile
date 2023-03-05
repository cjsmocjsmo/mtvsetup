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

FROM ubuntu:22.04

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get -y autoclean && \
  apt-get -y autoremove

WORKDIR /root/

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup

RUN \
  mkdir ./static && \
  mkdir ./fsData && \
  mkdir ./fsData/music && \
  mkdir ./fsData/music/music && \
  mkdir ./fsData/music/thumbnails && \
  mkdir ./fsData/music/metadata && \
  chmod -R +rwx ./static && \
  chmod -R +rwx ./fsData && \
  chmod -R +rwx ./fsData/music && \
  chmod -R +rwx ./fsData/music/music && \
  chmod -R +rwx ./fsData/music/thumbnails && \
  chmod -R +rwx ./fsData/music/metadata && \
  mkdir ./fsData/movies && \
  mkdir ./fsData/movies/movies && \
  mkdir ./fsData/movies/thumbnails && \
  mkdir ./fsData/movies/metadata && \
  chmod -R +rwx ./fsData/movies && \
  chmod -R +rwx ./fsData/movies/movies && \
  chmod -R +rwx ./fsData/movies/thumbnails && \
  chmod -R +rwx ./fsData/movies/metadata

STOPSIGNAL SIGINT

CMD ["mtvsetup"]
