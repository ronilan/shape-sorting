FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/shape-sorting/releases/latest/download/shape_sorting-terminal-linux.zip
    -o /tmp/shape_sorting.zip
    && unzip -o /tmp/shape_sorting.zip -d /usr/local/bin
    && rm /tmp/shape_sorting.zip
    && chmod +x /usr/local/bin/shape_sorting

CMD ["/bin/bash", "-i"]