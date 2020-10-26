#!/bin/sh

cd nodejs
neon clean
neon build --release
cd ../
