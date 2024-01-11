# my rcore os



# how to run os ?

## run
``` bash
cd os

make run_os

```

## run with debug

``` bash
cd os

make run_os_debug

gdb-multiarch \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

```