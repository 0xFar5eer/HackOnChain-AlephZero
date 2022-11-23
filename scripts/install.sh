apt install curl git; curl https://sh.rustup.rs -sSf | sh; source "$HOME/.cargo/env"; rustup toolchain add nightly;  apt install -y binaryen openssl;  cargo install cargo-dylint dylint-link;  cargo install cargo-contract --force --locked;  rustup component add rust-src --toolchain nightly;  rustup target add wasm32-unknown-unknown --toolchain nightly;  
git clone 
https://github.com/0xFar5eer/HackOnChain-AlephZero/;  cd HackOnChain*/scripts;  bash deploy_testnet.sh
