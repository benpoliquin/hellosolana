# helloworld for the Solana blockchain

## About

Currently teaching myself Solana development and wanted to start off with a simple hello world program using [Anchor](https://project-serum.github.io/anchor/getting-started/introduction.html). 
This Solana program runs locally and is able to create a counter value named `total` for an account and add to the counter for the connected account. (Read & Write on the blockchain)

This currently prints the value of `total` initially, adds one, and finally outputs the programs tx signature.

### To setup the local environment:
* [Gist](https://gist.github.com/benpoliquin/cf545b468040069609770dddc3534055)

### Execute test:

```
# In two seperate shells
solana-test-validator --no-bpf-jit
anchor test --skip-local-validator
```

### Test output:

```
BPF SDK: /Users/benpoliquin/workspace/solana/bin/sdk/bpf
Running: rustup toolchain list -v
Running: cargo +bpf build --target bpfel-unknown-unknown --release
    Finished release [optimized] target(s) in 0.24s
Running: /Users/benpoliquin/workspace/solana/bin/sdk/bpf/dependencies/bpf-tools/llvm/bin/llvm-readelf --dyn-symbols /Users/benpoliquin/workspace/hellosolana/target/deploy/helloworld.so

To deploy this program:
  $ solana program deploy /Users/benpoliquin/workspace/hellosolana/target/deploy/helloworld.so
The program address will default to this keypair (override with --program-id):
  /Users/benpoliquin/workspace/hellosolana/target/deploy/helloworld-keypair.json
Deploying workspace: http://localhost:8899
Upgrade authority: /Users/benpoliquin/.config/solana/id.json
Deploying program "helloworld"...
Program path: /Users/benpoliquin/workspace/hellosolana/target/deploy/helloworld.so...
Program Id: EHyhQcq2wU6Vtavi8eJBUis4PnfcAuKU2tMNzG7LK7hy

Deploy success
Starting the test!


  0 passing (0ms)

ACCOUNT TOTAL -- { total: <BN: 0> }
ACCOUNT TOTAL -- { total: <BN: 1> }
Sig -- 2u1iUpz3YBwXdhHbq1MR9EzaCuCQsa87yeMaPM3PdJbonPJHdubAcptQb27Cd5T268AgxrcCmGNE3atagDHz5bMA
```