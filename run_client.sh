#!/usr/bin/bash

cargo build --release --target=x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    wine ./target/x86_64-pc-windows-gnu/release/osd-client.exe
else
    echo "Application failed to build, will not run"
fi
