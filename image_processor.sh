#!/usr/bin/env bash

cargo run --quiet > image.hex && xxd -r -p image.hex image.bmp