#!/bin/sh
gcc -Wall -O2 -c start.S mode64.S
xargo build --target=x86_64-mros
ld -n --gc-sections -o kernel -T ./kernel.lds start.o mode64.o ./target/x86_64-mros/debug/libmros.a
