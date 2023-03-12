FROM rust:buster AS builder

RUN mkdir /usr/src/mtvsetup
RUN mkdir /usr/src/mtvsetup/src
RUN mkdir /usr/src/mtvsetup/target

COPY .env /root/

WORKDIR /usr/src/mtvsetup

COPY Cargo.toml .

COPY src/config.yaml ./src
COPY src/main.rs ./src
COPY src/mtv_clean.rs ./src
COPY src/mtv_create_ids.rs ./src
COPY src/mtv_env_vars.rs ./src
COPY src/mtv_image.rs ./src
COPY src/mtv_misc.rs ./src
COPY src/mtv_mp3_info.rs ./src
COPY src/mtv_nnc_info.rs ./src
COPY src/mtv_process_movie_images.rs ./src
COPY src/mtv_process_movies.rs ./src
COPY src/mtv_process_music_images.rs ./src
COPY src/mtv_process_music.rs ./src
COPY src/mtv_process_tvshows.rs ./src
COPY src/mtv_split.rs ./src
COPY src/mtv_walk_dirs.rs ./src

RUN cargo install --path .

# FROM ubuntu:22.04
FROM debian:buster

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get -y autoclean && \
  apt-get -y autoremove

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup

WORKDIR /root/

RUN \
  mkdir ./static && \
  mkdir ./fsData && \
  mkdir ./fsData/music && \
  mkdir ./fsData/thumbnails && \
  mkdir ./fsData/metadata

RUN \
  chmod -R +rwx ./static && \
  chmod -R +rwx ./fsData && \
  chmod -R +rwx ./fsData/music && \
  chmod -R +rwx ./fsData/thumbnails && \
  chmod -R +rwx ./fsData/metadata

RUN \
  mkdir ./fsDataMov && \
  mkdir ./fsDataMov/movies && \
  mkdir ./fsDataMov/thumbnails && \
  mkdir ./fsDataMov/metadata && \
  mkdir ./fsDataMov/posters

RUN \
  chmod -R +rwx ./fsDataMov/movies && \
  chmod -R +rwx ./fsDataMov/thumbnails && \
  chmod -R +rwx ./fsDataMov/metadata && \
  chmod -R +rwx ./fsDataMov/posters


RUN \
  mkdir ./fsDataTVShows && \
  mkdir ./fsDataTVShows/tvshows && \
  mkdir ./fsDataTVShows/thumbnails && \
  mkdir ./fsDataTVShows/metadata && \
  mkdir ./fsDataTVShows/posters


RUN \
  chmod -R +rwx ./fsDataTVShows && \
  chmod -R +rwx ./fsDataTVShows/tvshows && \
  chmod -R +rwx ./fsDataTVShows/thumbnails && \
  chmod -R +rwx ./fsDataTVShows/metadata && \
  chmod -R +rwx ./fsDataTVShows/posters


STOPSIGNAL SIGINT

# CMD ["tail", "-f", "/dev/null"]
CMD ["mtvsetup"]
