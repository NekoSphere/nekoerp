FROM ubuntu:24.04.3

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        bash \
        sudo \
        curl \
        ca-certificates \
        iproute2 \
        procps \
        net-tools \
        vim-tiny \
        redis-server \
        postgresql postgresql-contrib \
        nginx \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash ubuntu && \
    echo "ubuntu ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER ubuntu
WORKDIR /home/ubuntu

CMD ["bash", "-c", "sudo service postgresql start && sudo service redis-server start && sudo service nginx start && /bin/bash"]