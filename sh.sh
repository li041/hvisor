cargo clean
make clean
make BID=aarch64/rk3588
make BID=aarch64/rk3588 gen-fit 
#cp ./target/aarch64-unknown-none/debug/hvisor.bin ~/tftp
