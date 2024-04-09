export RUST_BACKTRACE := "1"
alias s:= setup 
alias h:= set-hooks
alias c:= clean
alias cov:= coverage

@setup:
    # rustup install nightly
    cargo install cargo-tarpaulin
    rustup component add clippy-preview
    pip install pre-commit
    just h
    cargo build

@clean:
    rm -rf pkg target  dist 

@set-hooks:
    pre-commit
    bash ./scripts/pre-push

@coverage:
    cargo +nightly tarpaulin --verbose --all-features --workspace --timeout 120 --out xml