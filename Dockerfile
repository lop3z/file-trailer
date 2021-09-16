FROM busybox:latest
ARG APP=log-bouncer
ENV APP=${APP}
COPY ${APP} .