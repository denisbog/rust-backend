from amazonlinux:latest

run yum update && yum install gcc openssl-devel -y
user 1000:1000
env HOME=/tmp
run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
env PATH="$HOME/.cargo/bin:$PATH" 
workdir /app
cmd cargo b --release
