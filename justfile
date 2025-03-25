
alias f := fmt
alias l := lint

# Install interal tools to manage the repository
install-tools:
  cargo install cargo-binstall
  cargo binstall taplo-cli knope

# Format files
fmt:
 cargo fmt
 taplo format

# Lint files
lint:
 cargo clippy --all-targets