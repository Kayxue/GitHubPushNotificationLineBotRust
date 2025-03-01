ARG ARCH=
FROM ${ARCH}rust:alpine AS build
WORKDIR /src
COPY . .

RUN USER=root apk add pkgconfig openssl-dev libc-dev ca-certificates
RUN cargo build --release

FROM scratch
WORKDIR /
COPY --from=build /src/target/release/GitHubPushNotificationLineBotRust ./serve
COPY --from=build /etc/ssl/certs /etc/ssl/certs

EXPOSE 3000

ENTRYPOINT ["./serve"]