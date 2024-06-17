USERNAME="iankressin"
REPO_NAME="tx-script"
TXSUP_URL="https://raw.githubusercontent.com/${USERNAME}/${REPO_NAME}/main/txsup/txsup.sh"

initial_message() {
    echo "
    ████████╗██╗  ██╗███████╗ ██████╗██████╗ ██╗██████╗ ████████╗
    ╚══██╔══╝╚██╗██╔╝██╔════╝██╔════╝██╔══██╗██║██╔══██╗╚══██╔══╝
       ██║    ╚███╔╝ ███████╗██║     ██████╔╝██║██████╔╝   ██║   
       ██║    ██╔██╗ ╚════██║██║     ██╔══██╗██║██╔═══╝    ██║   
       ██║   ██╔╝ ██╗███████║╚██████╗██║  ██║██║██║        ██║   
       ╚═╝   ╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝        ╚═╝ 
    "

    echo "[INFO] Installing txsup, the version manager of TXScript"
}

remove_old_version() {
    echo "[INFO] Removing old version of txsup"
    sudo rm -f /usr/local/bin/txsup
    echo "[INFO] Old version removed "
}

download_txsup() {
    curl -s -o txsup.sh $TXSUP_URL
    chmod +x txsup.sh
}

move_txsup() {
    sudo mv txsup.sh /usr/local/bin/txsup
}

final_message() {
    echo "---------------------- Installation complete ----------------------"
    echo ">>> Run 'txsup' to install TXScript"
}

initial_message
remove_old_version
download_txsup
move_txsup
final_message
