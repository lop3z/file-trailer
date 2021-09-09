FROM scratch
ARG APP=log-bouncer
ENV APP=${APP}
COPY ${APP} .