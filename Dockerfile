FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/incredible_app_template1/releases/latest/download/incredible_app_template1-terminal-linux.zip \
    -o /tmp/incredible_app_template1.zip \
    && unzip -o /tmp/incredible_app_template1.zip -d /usr/local/bin \
    && rm /tmp/incredible_app_template1.zip \
    && chmod +x /usr/local/bin/incredible_app_template1

CMD ["/bin/bash", "-i"]
