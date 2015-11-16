echo off
pushd pipe-consumer
cargo build
popd
pushd pipe-producer
cargo build
popd
xcopy /y pipe-consumer\target\debug\pipe-consumer.exe .\
xcopy /y pipe-producer\target\debug\pipe-producer.exe .\

pushd socket-consumer
cargo build
popd
pushd socket-producer
cargo build
popd
xcopy /y socket-consumer\target\debug\socket-consumer.exe .\
xcopy /y socket-producer\target\debug\socket-producer.exe .\

pushd message-consumer
cargo build
popd
pushd message-producer
cargo build
popd
xcopy /y message-consumer\target\debug\message-consumer.exe .\
xcopy /y message-producer\target\debug\message-producer.exe .\

pushd sharedmem-consumer
cargo build
popd
pushd sharedmem-producer
cargo build
popd
xcopy /y sharedmem-consumer\target\debug\sharedmem-consumer.exe .\
xcopy /y sharedmem-producer\target\debug\sharedmem-producer.exe .\