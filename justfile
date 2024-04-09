export RUST_BACKTRACE := "1"
alias s:= setup 
alias h:= set-hooks
alias c:= clean

@setup:
    # rustup install nightly
    rustup component add clippy-preview
    pip install pre-commit
    just h
    cargo build

@clean:
    rm -rf pkg target  dist 

@set-hooks:
    pre-commit
    bash ./hooks/pre-push