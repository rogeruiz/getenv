# Get Env

[![Build Status](https://travis-ci.org/rogeruiz/getenv.svg?branch=master)](https://travis-ci.org/rogeruiz/getenv)

A simple little CLI tool for getting the environment variable as an `export`
statement for any name you pass as the first argument.

## Compiling getenv from source

```shell
$ git clone https://github.com/rogeruiz/getenv.git
$ cd getenv
$ cargo build --release
$ cp ./target/release/getenv /usr/local/bin/getenv
```

## Usage

```shell
# if the variable exists
$ getenv DATABASE_URL > exports.txt
$ cat exports.txt
export DATABASE_URL="path/to/db"

# if the variable doesn't exist
$ getenv DATA_URL > exports.txt
$ cat exports.txt
DATA_URL, environment variable not found
```
