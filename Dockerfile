FROM rust:1.43.1

WORKDIR /build

RUN apt-get update && apt-get install -y gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf

CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf", "--release"]