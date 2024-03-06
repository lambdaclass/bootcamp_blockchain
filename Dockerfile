FROM ignitehq/cli as builder

USER root
WORKDIR /usr/src/app

COPY . .

RUN ignite chain build

ENTRYPOINT [ "lambchaind" ]
