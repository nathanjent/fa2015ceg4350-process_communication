echo off
pushd pipe-consumer
cargo build
popd
pushd pipe-producer
cargo build
popd
xcopy /y pipe-consumer\target\debug\pipe-consumer.exe .\
xcopy /y pipe-producer\target\debug\pipe-producer.exe .\