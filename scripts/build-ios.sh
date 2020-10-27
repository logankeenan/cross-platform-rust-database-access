#!/bin/sh

cd ./rust-ios

cargo clean
cbindgen src/lib.rs -l c > rust-ios.h
cargo lipo --release

cd ../ios
rm -rf rust-ios

mkdir -p rust-ios/libs
mkdir -p rust-ios/include

cd ../
cp rust-ios/rust-ios.h ios/rust-ios/include
cp rust-ios/target/universal/release/librust_ios.a ios/rust-ios/libs

