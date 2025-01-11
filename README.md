```commandline
     _                              _           _
    | |_   _ _ __   __ _  ___   ___| |__   __ _(_)_ __
 _  | | | | | '_ \ / _` |/ _ \ / __| '_ \ / _` | | '_ \
| |_| | |_| | | | | (_| | (_) | (__| | | | (_| | | | | |
 \___/ \__,_|_| |_|\__, |\___/ \___|_| |_|\__,_|_|_| |_|
                   |___/

```

# **Jungochain** <!-- omit in toc -->
<!-- TODO -->
<!-- [![Discord Chat](https://img.shields.io/discord/308323056592486420.svg)](https://discord.gg/jungochain) -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/jungoai/jungochain?tab=MIT-1-ov-file)

It's the blockchain layer of Jungoai.

## Run a node

- System Requirements
  - RAM: ~286 MiB
  - Architectures:
    - Linux x86_64:
      - Linux kernel 2.6.32+,
      - glibc 2.11+
    - MacOS x86_64:
      - MacOS 10.7+ (Lion+)
- Network requirements
  - Jungochain needs access to the public internet
  - Jungochain runs on ipv4
  - Jungochain listens on the following ports:
    - 9944 - Websocket. This port is used by jungo-sdk and jungo-cli. It only accepts connections from localhost. Make sure this port is firewalled off from the public domain unless you wanted to run a RPC node.
    - 9933 - RPC. This port is opened, but not used.
    - 30333 - p2p socket. This port accepts connections from other jungochain nodes. Make sure your firewall(s) allow incoming traffic to this port.
  - It is assumed your default outgoing traffic policy is ACCEPT. If not, make sure outbound traffic to port 30333 is allowed.

### Login to ghcr.io

- Generate a PAT:
  - Go to [GitHub Developer Settings](https://github.com/settings/tokens).
  - Click *Generate new token* (classic).
  - Select scopes: read:packages (to pull images).
- Login to ghcr.io
```bash
echo "<your_personal_access_token>" | docker login ghcr.io -u <your_github_username> --password-stdin
```

### Pull docker image:

```bash
docker pull ghcr.io/jungoai/jungochain:0.1.0-devnet
```

### Run image

Simply run:

```bash
. ./scripts/bootstrap/run_by_docker.sh
```

Or if you want this options run:

```bash
# --name <NAME>           # (optional) show your node name in telemetry
# --telemetry-url <ADDR>  # (optional) telemetry address
# --rpc                   # (optional) expose rpc to external requests
# --archive               # (optional) if you want to run archive node (it needs about 1.5 TB storage)
. ./scripts/bootstrap/run_by_docker.sh --name your-disired-name --rpc --archive --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

Note: chain data would be store on "$HOME/.jungochain/" directory

### Ensure running image

Check running container:
```bash
docker ps
```

It should show something like:
```bash
CONTAINER ID   IMAGE                                     COMMAND  CREATED          STATUS          PORTS     NAMES
ae61c5ea3863   ghcr.io/jungoai/jungochain:0.1.0-devnet   ...      13 seconds ago   Up 12 seconds             angry_perlma
```

Check the logs (use your container id instead of `ae61c5ea3863`):
```bash
docker logs ae61c5ea3863
```

You should see a line like:
```bash
🔍 Discovered new external address for our node: ...
```

You can stop the service whenever you want with:
```bash
docker stop <CONTAINER_ID>
```

## Run a Subnet

Checkout [Jungo Echo Subnet](https://github.com/jungoai/jungo-echo-subnet) example.

## For Jungochain Development

Install Rust:
- Method 1: through [rust installation doc](https://www.rust-lang.org/tools/install)
- Method 2: uncomment `rustup` in `flake.nix` file

Install Nix:

```sh
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

Then enter to development environment from the root of the project folder (where flake.nix exist):

```sh
nix develop
```

### Build

Use the following command to build the node without launching it:

```sh
cargo build -p jungochain-node --release
```

Or to enable local faucet:
```sh
cargo build -p jungochain-node --release --features pow-faucet
```

For development it's good to enable fast-blocks to reduce the time of block creation:
```sh
cargo build -p jungochain-node --release --features pow-faucet,fast-blocks
```

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/jungochain-node --dev
```

Running debug with logs.
```bash
RUST_LOG=runtime=debug ./target/release/jungochain-node -- --nocapture
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/jungochain-node -ldebug --dev
```

Running individual tests
```bash
SKIP_WASM_BUILD=1 \
  RUST_LOG=runtime=debug \
  cargo test <your test name> \
  -- --nocapture --color always
```

<details>
  <summary>testing `tests/` tips</summary>

  **`<package-name>`**
  Available members are found within the project root [`./cargo.toml`](./cargo.toml) file, each
  point to a sub-directory containing a `cargo.toml` file with a `name` defined.  for example,
  [`node/cargo.toml`](./node/cargo.toml) has a name of `jungochain-node`


  **`<test-name>`**
  Available tests are often found within either a `tests/` sub-directory or within the relevant
  `src/` file.  for example [`./node/tests/chain_spec.rs`](./node/tests/chain_spec.rs) has a test
  named `chain_spec`

  **example**
  All together we can run all tests in `chain_spec` file from `jungochain-node` project via

  ```bash
  skip_wasm_build=1 \
    rust_log=runtime=debug \
    cargo test \
    --package jungochain-node \
    --test chain_spec \
    -- --color always --nocapture
  ```
</details>


<!-- TODO -->
<!-- Running code coverage -->
<!-- ```bash -->
<!-- bash scripts/code-coverage.sh -->
<!-- ``` -->

<!-- > Note: They above requires `cargo-tarpaulin` is installed to the host, eg. `cargo install cargo-tarpaulin` -->
<!-- > Development chain means that the state of our chain will be in a tmp folder while the nodes are -->
<!-- > running. Also, **alice** account will be authority and sudo account as declared in the -->
<!-- > [genesis state](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/node/src/chain_spec.rs#L49). -->
<!-- > At the same time the following accounts will be pre-funded: -->
<!-- > - Alice -->
<!-- > - Bob -->
<!-- > - Alice//stash -->
<!-- > - Bob//stash -->

If we want to maintain the chain state between runs, a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands show how to use a newly created folder as our db base path:

```bash
# Create a folder to use as the db base path
mkdir my-chain-state

# Use of that folder to store the chain state
./target/release/jungochain-node --dev --base-path ./my-chain-state/

# Check the folder structure created inside the base path after running the chain
ls ./my-chain-state
#> chains
ls ./my-chain-state/chains/
#> dev
ls ./my-chain-state/chains/dev
#> db keystore network
```

**Connect with Polkadot-JS Apps Front-end**

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### CLI help

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/jungochain-node --help
```

## For Subnet Development

Checkout [Jungo SDK](https://github.com/jungoai/jungo-sdk).
