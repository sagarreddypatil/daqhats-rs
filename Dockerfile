FROM --platform=linux/arm64 debian:bookworm

RUN apt-get update && apt-get install -y \
    gpiod \
    libgpiod-dev \
    build-essential \
    git

RUN git clone --depth 1 --branch v1.5.0.0 https://github.com/mccdaq/daqhats.git /daqhats
WORKDIR /daqhats

RUN make -C lib all
RUN make -C lib install
RUN make -C lib clean

RUN apt-get install curl clang libclang-dev -y

RUN groupadd -g 1000 dev && \
    useradd -u 1000 -g dev -ms /bin/bash dev

USER dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
