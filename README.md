# 2 nodes chillin

But in the middle we stay calm, we just drop bombs
Asking where we come from? South Coast slums
It's just two dope boys in a cadillac
It's just two dope boys in a cadillac

## LDK Node Experiment

This creates 2 nodes connected to Mutinynet, but keeps them behind local IPs for testing between each other.

There is `node1` and `node2`. You can issue commands via the interactive CLI.

### Commands

- `balance`
- `openchannel`
- `getinvoice`
- `payinvoice`

### Example

- `cargo run` or `cargo build --release` and then run `./target/release/ldk-node-test`
- `node1 getaddress`
- Paste the output into [Mutinynet Faucet](https://faucet.mutinynet.com/) to get test sats
- `node1 balance` to confirm your balance is there
- `node1 openchannel` to open a channel to node2
- `node2 getinvoice` to get an invoice
- `node1 pay invoice lntb...` to pay the invoice
- `node1 balance` to confirm it worked


