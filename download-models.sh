#!/bin/bash

script="https://raw.githubusercontent.com/ggerganov/whisper.cpp/refs/heads/master/models/download-ggml-model.sh"

# run the script with the arguments passed to this script

mkdir -p models

cd models || exit 1

curl -s $script | bash -s "$@"
