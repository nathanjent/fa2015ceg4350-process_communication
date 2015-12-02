#! /bin/sh
pushd pipe-consumer
cargo build
popd
pushd pipe-producer
cargo build
popd
cp pipe-consumer/target/debug/pipe-consumer ./target
cp pipe-producer/target/debug/pipe-producer ./target

pushd socket-consumer
cargo build
popd
pushd socket-producer
cargo build
popd
cp socket-consumer/target/debug/socket-consumer ./target
cp socket-producer/target/debug/socket-producer ./target

pushd message-consumer
cargo build
popd
pushd message-producer
cargo build
popd
cp message-consumer/target/debug/message-consumer ./target
cp message-producer/target/debug/message-producer ./target

pushd sharedmem-consumer
cargo build
popd
pushd sharedmem-producer
cargo build
popd
cp sharedmem-consumer/target/debug/sharedmem-consumer ./target
cp sharedmem-producer/target/debug/sharedmem-producer ./target
