FROM ubuntu:20.04
ENV DEBIAN_FRONTEND noninteractive
RUN apt update -y && \
    apt install -y bison bc make python-is-python3 libncurses-dev libssl-dev libelf-dev flex curl git gcc && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \    
   . $HOME/.cargo/env && git clone https://github.com/docfate111/penguincrab.git && \
   cd penguincrab && \
   cargo build && cargo test -- --nocapture 
