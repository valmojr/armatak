# build Golang app for Linux
FROM golang:latest

WORKDIR /app

# get gcc-multilib and gcc-mingw-w64
RUN apt update
RUN apt install -y gcc-multilib gcc-mingw-w64

CMD ["/bin/sh"]