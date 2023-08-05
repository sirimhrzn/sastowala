FROM rust:1.71 

WORKDIR /var/www
COPY . .


RUN cargo build --release

ENTRYPOINT ["cargo","run","--release"]