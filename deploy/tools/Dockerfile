# Simple dockerfile to set up an environment for
# reliably building binaries for running on lambda
FROM amazonlinux:2

RUN yum update -y
RUN yum install -y git gcc
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /root
COPY build-release-binary-internal.sh .
RUN chmod +x build-release-binary-internal.sh
