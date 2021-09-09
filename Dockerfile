FROM debian:8
EXPOSE 8080
CMD ["/log-bouncer"]
COPY target/release/ /
