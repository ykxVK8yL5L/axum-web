FROM alpine:latest
ARG TARGETARCH
ARG TARGETVARIANT
RUN apk --no-cache add ca-certificates tini
RUN apk add tzdata && \
	cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
	echo "Asia/Shanghai" > /etc/timezone && \
	apk del tzdata

RUN mkdir -p /etc/axum-web
WORKDIR /root/
ADD axum-web-$TARGETARCH$TARGETVARIANT /usr/bin/axum-web

ENTRYPOINT ["/sbin/tini", "--"]
#CMD ["/usr/bin/axum-web", "--host", "0.0.0.0", "--workdir", "/etc/axum-web"]
CMD ["/usr/bin/axum-web"]
