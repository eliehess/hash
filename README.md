# hash

Computes SHA-256 hash of either raw input or the contents of a given file

## Usage:
Raw input with `-r` or `--raw`
```
> hash -r "Hello, world!"
315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3
```
Contents of a file with `-f` or `--file`
```
> cat .\test.txt
This is a test
> hash --file .\test.txt
b6c7de29c2be4ccb60bf7c9f65c8b8f2be8345c0a78beb03eaee96f022e37a5d
```