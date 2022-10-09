# APSS
This repository contains the code for an asynchronous proactive secret sharing protocol which is currently in submission. Please read the corresponding section in the paper to get an idea what this prototype implements.

## Structure
* `utils` offers some useful macros (primarily for dealing with `tokio` channels).
* `network` handles asynchronous network conditions, e.g., retrying after transmission failures, caching messages which are not yet required. It offers a pub-sub-style interface for sub-protocols.
* `protocol` offers common traits that describe protocols.
* `crypto` offers some cryptography traits.
* `crypto_blstrs` implements these traits and also offers additional constructions (e.g., KZG commitments).
* `tss` is a simple threshold signing protocol
* `vaba`, `acss` and `apss` implement the (sub-)protocols as described in the paper.
* `cli` is a CLI interface for APSS. After building, run `cli --help` to learn more.

## Running this software
This software can be compiled using `cargo` and run on any machine that does not have an unreasonably old CPU. Note that it was only tested on Linux.
