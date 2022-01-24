# hash

Computes hashes of either raw input or the contents of a given file

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
Select hashing algorithm with `-a` or `--algorithm`. Defaults to SHA-256 if not specified
```
> hash --raw "Hello, world!" -a sha512
c1527cd893c124773d811911970c8fe6e857d6df5dc9226bd8a160614c0cd963a4ddea2b94bb7d36021ef9d865d5cea294a82dd49a0bb269f51f6e7a57f79421
```