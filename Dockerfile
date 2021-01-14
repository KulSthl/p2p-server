FROM alpine:latest

COPY ./target/x86_64-unknown-linux-musl/release ./usr/local/app/server
COPY ./migrations ./usr/local/app/server/migrations
COPY ./wwwroot ./usr/local/app/server/wwwroot
COPY ./release_settings.cfg ./usr/local/app/server/.env
WORKDIR /usr/local/app/server
EXPOSE 8080

CMD ["./p2p_server"]