#!/bin/bash

USERNAME="iankressin"
REPO_NAME="tx-script"
REPO_URL="https://github.com/{$USERNAME}/${REPO_NAME}"
REPO_API_URL="https://api.github.com/repos/${USERNAME}/${REPO_NAME}"

LINUX_ASSET="x86_64-unknown-linux-musl.zip"
MAC_ASSET="x86_64-apple-darwin.zip"

get_latest_release_tag() {
    LATEST_RELEASE_TAG=$(curl -s "${REPO_API_URL}/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')
}

initial_message() {
    echo "
    ████████╗██╗  ██╗███████╗██╗   ██╗██████╗ 
    ╚══██╔══╝╚██╗██╔╝██╔════╝██║   ██║██╔══██╗
       ██║    ╚███╔╝ ███████╗██║   ██║██████╔╝
       ██║    ██╔██╗ ╚════██║██║   ██║██╔═══╝ 
       ██║   ██╔╝ ██╗███████║╚██████╔╝██║     
       ╚═╝   ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝

     ((( The tx-script version manager )))
    "

    echo "[INFO] Installing the lastest version of tx-script: $LATEST_RELEASE_TAG"
}

detect_os() {
    if [ "$OSTYPE" == "linux-gnu" ]; then
        ASSET_NAME=$LINUX_ASSET
        echo "[INFO] Linux detected"
    elif [ "$OSTYPE" == "darwin"* ]; then
        ASSET_NAME=$MAC_ASSET
        echo "[INFO] MacOS detected"
    elif [ "$OSTYPE" == "cygwin" ]; then
        echo "[INFO] On Windows, download the executable from the link below:"
        echo "{ $REPO_URL }/releases/latest"
        exit 1
    else
        echo "[INFO] Unsupported OS"
        exit 1
    fi
}

download_asset() {
    echo "[INFO] Downloading asset"
    curl -L -o latest.zip "${REPO_URL}/releases/download/${LATEST_RELEASE_TAG}/${ASSET_NAME}"
    echo "[INFO] Asset downloaded"
}

extract_asset() {
    echo "[INFO] Extracting assets"
    unzip latest.zip -d latest > /dev/null

    if [ $? -ne 0 ]; then
        echo "[INFO] Failed to unzip asset"
        exit 1
    fi
    echo "[INFO] Assets extracted"
}
 
move_to_bin() {
    echo "[INFO] Moving to /usr/local/bin"
    sudo mv latest/tx-script /usr/local/bin/txs
    chmod +x /usr/local/bin/txs
    echo "[INFO] Installed to /usr/local/bin/txs"
}

cleanup() {
    rm -rf latest latest.zip
    echo "[INFO] Cleaned up"
}

remove_old_version() {
    echo "[INFO] Removing old version of tx-script"
    sudo rm -f /usr/local/bin/txs
    echo "[INFO] Old version removed "
}

final_message() {
    echo "---------------------- Installation complete ----------------------"
    echo ">>> Run 'txs --help' to get started"
}

get_latest_release_tag
initial_message
remove_old_version
detect_os
download_asset
extract_asset
move_to_bin
cleanup
final_message
