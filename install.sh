#!/usr/bin/env bash

# A script to download and install the latest (or specified) version
# of the Stencila CLI on MacOS or Linux
#  
# First arg:  desired version e.g. v2.1.0
#             defaults to latest
# Second arg: desired install location, e.g. /bin
#             defaults to /usr/local/bin on MacOS, to ~/.local/bin on Linux

set -e

OS=$(uname)
if [[ "${OS}" == "Linux" || "${OS}" == "Darwin" ]]; then
    case "${OS}" in
        'Linux')
            TARGET_TRIPLE="x86_64-unknown-linux-gnu"
            VERSION="${1:-latest}"
            if [ "$VERSION" == "latest" ]; then
                VERSION=$(curl --silent "https://api.github.com/repos/stencila/stencila/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')
            fi
            INSTALL_PATH="${2:-$HOME/.local/bin}"
            ;;
        'Darwin')
            TARGET_TRIPLE="x86_64-apple-darwin"
            if [ "$VERSION" == "latest" ]; then
                VERSION=$(curl --silent "https://api.github.com/repos/stencila/stencila/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
            fi
            INSTALL_PATH="${2:-/usr/local/bin}"
            ;;
    esac
    
    echo "Downloading stencila $VERSION for platform $TARGET_TRIPLE"
    curl -sL https://github.com/stencila/stencila/releases/download/$VERSION/stencila-$VERSION-$TARGET_TRIPLE.tar.gz | tar xz || {
        echo 
        echo "There was an error downloading stencila-$VERSION-$TARGET_TRIPLE.tar.gz"
        echo "It may be that binaries are not yet available yet."
        echo "Please wait, or try a previous version at https://github.com/stencila/stencila/releases."
        echo
        exit 1
    }
    
    echo "Installing stencila in $INSTALL_PATH"
    mkdir -p $INSTALL_PATH
    mv stencila* $INSTALL_PATH
    
    echo "Successfully installed stencila CLI"
else
    echo "Sorry, I don't know how to install on this operating system, please see https://github.com/stencila/stencila#readme"
fi
