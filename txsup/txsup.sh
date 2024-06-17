#!/bin/bash

# https://github.com/iankressin/tx-script/releases/download/0.1.0-beta/x86_64-unknown-linux-musl.zip

USERNAME="iankressin"
REPO_NAME="tx-script"
REPO_URL="https://github.com/{$USERNAME}/${REPO_NAME}"
REPO_API_URL="https://api.github.com/repos/${USERNAME}/${REPO_NAME}"

LINUX_ASSET="x86_64-unknown-linux-musl.zip"
MAC_ASSET="x86_64-apple-darwin.zip"

detect_os() {
    if [ "$OSTYPE" == "linux-gnu" ]; then
        ASSET_NAME="txsup-linux"
        echo ">>> Linux detected"
    elif [ "$OSTYPE" == "darwin"* ]; then
        ASSET_NAME="txsup-mac"
        echo ">>> MacOS detected"
    elif [ "$OSTYPE" == "cygwin" ]; then
        echo ">>> On Windows, download the executable from the link below:"
        echo "{ $REPO_URL }/releases/latest"
        exit 1
    else
        echo ">>> Unsupported OS"
        exit 1
    fi
}

get_latest_release_tag() {
    LATEST_RELEASE_TAG=$(curl -s "${REPO_API_URL}/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')
    echo ">>> Latest release: $LATEST_RELEASE_TAG"
}

download_asset() {
    curl -L -o latest.zip "${REPO_URL}/releases/download/${LATEST_RELEASE_TAG}/${LINUX_ASSET}"
}

extract_asset() {
    unzip lastest -d lastest

    if [ $? -ne 0 ]; then
        echo ">>> Failed to unzip asset"
        exit 1
    fi
}

move_to_bin() {
    mv lastest/tx-script /usr/local/bin/txs
    chmod +x /usr/local/bin/${ASSET_NAME}
    echo ">>> TXS installed successfully"
}

cleanup() {
    rm -rf lastest latest.zip
    echo ">>> Cleaned up"
}

detect_os
get_latest_release_tag
download_asset
extract_asset
