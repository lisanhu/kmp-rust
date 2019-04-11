# KMP algorithm in Rust
__author:__ [*Sanhu Li*](mailto:lisanhu2014@hotmail.com), *Sunita Chandrasekaran*

## Introduction
This is a simple implementation for the KMP string searching algorithm in Rust. The pre-processing for query is not implemented using the most efficient way, but it should be enough for small queries.

It also comes with a test program that will take a reference file (referred as *ref*), and a query file (referred as *qry*) as input.

## Build
```Shell
$ cargo build --release
```

## Usage of the testing program
### File formats
Both *ref* and *qry* must be ASCII plain text file.

*qry* must contain queries separated by lines. It's not allowed to have queries with new line character.

### Usage
```Shell
$ target/release/kmp ref qry
```

## Performance Evaluation
No performance evaluation at the moment.

## Contact
[Sanhu Li](mailto:lisanhu2014@hotmail.com)
