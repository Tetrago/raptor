alias t := test
alias b := build
alias l := lint
alias cov := coverage
alias html := coverage-html
alias open := coverage-html-open

llvm-cov-args := "--ignore-filename-regex '.*rustlib.*' --no-cfg-coverage-nightly"

[private]
default:
    @just --list

test:
    @cargo nextest run

lint:
    @cargo clippy

build:
    @cargo build

[group('examples')]
parse file:
    @cargo run --example parse {{file}}

[group('examples')]
lex file:
    @cargo run --example lex {{file}}

[group('coverage')]
coverage:
    @cargo llvm-cov nextest {{llvm-cov-args}}

[group('coverage')]
coverage-html:
    @cargo llvm-cov nextest --html {{llvm-cov-args}}

[group('coverage')]
coverage-html-open:
    @cargo llvm-cov nextest --open {{llvm-cov-args}}
