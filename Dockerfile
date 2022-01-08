FROM busybox
ARG TARGETARCH
ARG TARGETVARIANT
WORKDIR /root/
ADD axum-web-$TARGETARCH$TARGETVARIANT /root/axum-web
CMD ["/root/axum-web"]
