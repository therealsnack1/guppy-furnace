FROM golang:1.20-alpine AS cosmos_builder

RUN apk --no-cache add \
        ca-certificates \
        build-base \
        direnv \
        jq \
        git \
        libusb-dev \
        make \
        linux-headers\
        leveldb-dev

RUN go install github.com/hashrocket/ws@latest

WORKDIR /cosmos-blockchain


RUN git clone https://github.com/White-Whale-Defi-Platform/migaloo-chain . && git checkout v2.2.5

COPY download_wasm.sh /download_wasm.sh
RUN /download_wasm.sh

ENV LEDGER_ENABLED=false
ENV BUILD_TAGS=muslc
ENV LINK_STATICALLY=true

RUN sh -c "go mod download && make install"



FROM alpine:3.16
RUN apk --no-cache add \
        curl \
        jq \
        yq

COPY --from=cosmos_builder /go/bin/ /usr/local/bin

COPY config.sh /config.sh

COPY entrypoint.sh /usr/local/bin

ENTRYPOINT ["entrypoint.sh"]
CMD ["migalood", "start", "--pruning=nothing", "--minimum-gas-prices=0uwhale", "--rpc.laddr=tcp://0.0.0.0:26657"]