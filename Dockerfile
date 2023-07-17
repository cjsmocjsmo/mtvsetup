FROM rust:buster AS builder

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
FROM debian:bookworm

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get -y autoclean && \
  apt-get -y autoremove

COPY --from=builder /usr/src/mtvsetup/target/release/mtvsetup /usr/local/bin/mtvsetup

WORKDIR /root/

RUN \
  mkdir ./static && chmod -R +rwx ./static && \
  mkdir ./fsData && chmod -R +rwx ./fsData && \
  mkdir ./gzip && chmod -R +rwx ./gzip && \
  mkdir ./fsData/music && chmod -R +rwx ./fsData/music && \
  mkdir ./fsData/thumbnails && chmod -R +rwx ./fsData/thumbnails && \
  mkdir ./fsData/metadata && chmod -R +rwx ./fsData/metadata


RUN \
  mkdir ./fsDataMov && \
  mkdir ./fsDataMov/movies && chmod -R +rwx ./fsDataMov/movies && \
  mkdir ./fsDataMov/thumbnails && chmod -R +rwx ./fsDataMov/thumbnails && \
  mkdir ./fsDataMov/metadata && chmod -R +rwx ./fsDataMov/metadata && \
  mkdir ./fsDataMov/posters && chmod -R +rwx ./fsDataMov/posters


RUN \
  mkdir ./fsDataTVShows && chmod -R +rwx ./fsDataTVShows && \
  mkdir ./fsDataTVShows/tvshows && chmod -R +rwx ./fsDataTVShows/tvshows && \
  mkdir ./fsDataTVShows/thumbnails && chmod -R +rwx ./fsDataTVShows/thumbnails && \
  mkdir ./fsDataTVShows/metadata && chmod -R +rwx ./fsDataTVShows/metadata && \
  mkdir ./fsDataTVShows/posters && chmod -R +rwx ./fsDataTVShows/posters


STOPSIGNAL SIGINT

# CMD ["tail", "-f", "/dev/null"]
CMD ["mtvsetup"]
