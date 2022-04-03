# rpt

Commandline program that repeats a string by a given amount. Supports a separator string, reversing the input string and
can be used in pipes.


## Background

This is a clone of the ANSI-C implementation called [`repeat`](https://github.com/marcotrosi/repeat) created by **marcotrosi**.


## Installing

    cargo install --path .


## Examples

    $ rpt 3 foo
    foofoofoo

    $ echo 'foo' | rpt 3
    foofoofoo

    $ rpt -s ',' 3 foo
    foo,foo,foo

    $ rpt -- -2 foo
    oofoof

    $ rpt -e -s '\t' 3 foo
    foo   foo   foo


## Usage

    rpt [-v] [-h] [-n] [-e] [-s <sep>] repetitions [string]


## Options

    -s <sep>  optional separator string
    -n        do not output the trailing newline
    -e        interpret some few escape sequences (\\,\t,\n)
    -v        print version info
    -h        print help text
