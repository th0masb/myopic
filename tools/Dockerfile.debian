FROM debian:bullseye

WORKDIR /root
RUN apt-get update && apt-get install -y ca-certificates
COPY app .
CMD ./app

