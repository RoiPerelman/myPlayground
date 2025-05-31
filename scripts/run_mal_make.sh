#!/bin/bash

# Check if parameter is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <step>"
  exit 1
fi

cd ~/Other/mal || exit
rsync -av --delete ~/myPlayground/ ~/Other/mal/impls/rust/
make "test^rust^$1"
cd -
