---
id: devnet
title: Running a Devnet
sidebar_label: Devnet
---

[general tags]: # "guides, devnet local_devnet, snarkos"

A local devnet can be a heavyweight but reliable way to test your application on Aleo.

## Setup

The Leo CLI provides a helpful command to help startup a local devnet:

```bash
leo devnet --snarkos <SNARKOS> --snarkos-features test_network
```

The `<SNARKOS>` is the path to an installed binary of [**snarkOS**](https://github.com/ProvableHQ/snarkOS), the decentralized operating system that forms the backbone of the Aleo network.

If you don't have snarkOS installed, you can pass the `--install` flag and the CLI will automatically download, compile, and store the binary at the path specified by `<SNARKOS>`.

```bash
leo devnet --snarkos <SNARKOS> --snarkos-features test_network --install
```

:::info

Windows users will need to perform some additional steps in order for snarkOS to install properly:

1. Upon initially installing Rust, you should have been automatically prompted to install Visual Studio with the MSVC 2022 C++ build tools.
2. Open the Visual Studio Installer and install the C++ Clang Compiler for Windows and either the Windows 10 SDK or Windows 11 SDK (depending on your OS). Make a note of the installation path of the installed tool. It should be of the form `{PATH}\Microsoft Visual Studio\2022\BuildTools`.
3. Within the aforementioned build tools directory, you should find the location of a file called `libclang.dll`. For `x86`-based systems, this should be in the `VC\Tools\Llvm\bin` subdirectory. For `x64`-based systems, this should be in the `VC\Tools\Llvm\x64\bin` subdirectory.
4. Once you have the full path of `libclang.dll`, create the `LIBCLANG_PATH` environment variable for your system and set it to this path.
5. snarkOS should now compile and run properly.

:::

The `tmux` command will allow you to toggle between nodes in your local devnet. You can enable this by passing the `--tmux` flag upon startup:

```bash
leo devnet --snarkos <SNARKOS> --snarkos-features test_network --tmux
```

:::info
This feature is only available on Unix-based systems.
:::

You'll need to install the `tmux` package first:

<Tabs defaultValue="macos"
values={[
{ label: 'MacOS', value: 'macos' },
{ label: 'Ubuntu', value: 'ubuntu' },
]}>
<TabItem value="macos">

To install `tmux` on macOS, you can use the Homebrew package manager. If you haven't installed Homebrew yet, you can find instructions at their [website](https://brew.sh/). Once Homebrew is installed, run:

```bash
brew install tmux
```

</TabItem>
<TabItem value="ubuntu">
On Ubuntu and other Debian-based systems, you can use the apt package manager:

```bash
sudo apt update
sudo apt install tmux
```

</TabItem>
</Tabs>

Here are some useful (default) commands in `tmux`:

```bash
# To toggle to the next node in a local devnet
Ctrl+b n
# To toggle to the previous node in a local devnet
Ctrl+b p
# To scroll easily, press q to quit
Ctrl+b q
# To select a node in a local devnet
Ctrl+b w
# To select a node manually in a local devnet
Ctrl+b :select-window -t {NODE_ID}
# To stop a local devnet
Ctrl+b :kill-session
```

See the full [`leo devnet` CLI documentation](./../cli/07_devnet.md).

## Usage

When you start the devnet, the CLI will actually spin up a new instance of the blockchain from genesis via the snarkOS binary. This means that the chain will start at block 0 and consensus version 1, and the only program deployed will be `credits.aleo`.

The height of the chain will increase as blocks are produced. At various different heights, a new consensus version will activate, which will unlock various features that have been implemented as the Aleo network has matured. By default, the devnet will assume the predefined consensus heights from Testnet:

```rust
(ConsensusVersion::V1, 0),
(ConsensusVersion::V2, 2_950_000),
(ConsensusVersion::V3, 4_800_000),
(ConsensusVersion::V4, 6_625_000),
(ConsensusVersion::V5, 6_765_000),
(ConsensusVersion::V6, 7_600_000),
(ConsensusVersion::V7, 8_365_000),
(ConsensusVersion::V8, 9_173_000),
(ConsensusVersion::V9, 9_800_000),
(ConsensusVersion::V10, 10_525_000),
(ConsensusVersion::V11, 11_952_000),
```

Obviously you don't have time to wait for 10 million blocks to access a newer feature like program upgradability, so `leo devnet` provides a way to manually set the consensus heights via the `--consensus-heights` flag:

```bash
leo devnet --snarkos <SNARKOS> --snarkos-features test_network --consensus-heights 0,1,2,3,4,5,6,7,8,9,10
```

Note that if you want to access the latest features, the number of comma-separated arguments you pass to this flag must be exactly equal to the latest consensus version.

Each time you stop and restart the chain, the prior state and history will be saved. You can clear any prior history by passing the `--clear-storage` flag:

```bash
leo devnet --snarkos <SNARKOS> --snarkos-features test_network --clear-storage
```

Clearing the ledger history may be useful if you wish to redeploy your program without changing the name. However, this will erase all transaction history and start a new instance of the Aleo blockchain from genesis.

## Deploying and Executing

When deploying or executing programs on a local devnet, make sure that endpoint is set to `http://localhost:3030` rather than any external API endpoints. You can do this either by manually setting the `ENDPOINT` environment variable, by passing the `--endpoint http://localhost:3030` flag in the CLI, or by setting the `ENDPOINT` variable in a `.env` file within the root directory of your Leo project.

You will also need credits to fund transactions on the devnet. snarkOS automatically initializes four development accounts funded with Aleo credits that can be used for testing purposes.

```bash
# Account 0
APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw
aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px

# Account 1
APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh
AViewKey1pTzjTxeAYuDpACpz2k72xQoVXvfY4bJHrjeAQp6Ywe5g
aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t

# Account 2
APrivateKey1zkp2GUmKbVsuc1NSj28pa1WTQuZaK5f1DQJAT6vPcHyWokG
AViewKey1u2X98p6HDbsv36ZQRL3RgxgiqYFr4dFzciMiZCB3MY7A
aleo1ashyu96tjwe63u0gtnnv8z5lhapdu4l5pjsl2kha7fv7hvz2eqxs5dz0rg

# Account 3
APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs
AViewKey1iKKSsdnatHcm27goNC7SJxhqQrma1zkq91dfwBdxiADq
aleo12ux3gdauck0v60westgcpqj7v8rrcr3v346e4jtq04q7kkt22czsh808v2
```

You can specify the private key to use by manually setting the `PRIVATE_KEY` environment variable, by passing the `--private-key http://localhost:3030` flag in the CLI, or by setting the `PRIVATE_KEY` variable in a `.env` file within the root directory of your Leo project.

Once your private key and endpoint have been correctly set, deploying and executing largely function the same as they would on Testnet or Mainnet. For more details on either of those processes, check out the [**Deploying**](./03_deploying.md) and [**Executing**](./04_executing.md) guides.

## Querying Transaction Status

You can check your transactions by using the following API endpoint:

```bash
GET http://localhost:3030/testnet/transaction/{TRANSACTION_ID}
```

or by using `leo query` from the CLI:

```bash
leo query transaction {TRANSACTION_ID}
```

The transaction API endpoint is instructive in verifying whether a transaction succeeded or failed. Since both successful and failed transactions execute a fee transaction, if only the fee transaction appears, that is a clear indication that the transaction has failed. Note that on the Testnet and on Mainnet, failed transactions still require a fee since the network is performing a computation.

A [full list of API endpoints](https://docs.explorer.provable.com/docs/api/v2/intro) is available in the explorer documentation.

## Record Scanning

You can use the CLI from your installed snarkOS binary to view your records. First, navigate to the directory where you installed the binary. Then use the following command syntax:

```bash
./snarkos developer scan --endpoint http://localhost:3030 --private-key {YOUR_PRIVATE_KEY} --start <block_number> --network 1
```

Setting `block_number` to `0` will list all of the records created starting from the genesis block, including your test credit records.

```bash title="sample output:"
⚠️  Attention - Scanning the entire chain. This may take a while...

Scanning 3 blocks for records (100% complete)...

[
  "{  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,  microcredits: 23437500000000u64.private,  _nonce: 3666670146276262240199958044811329632452609778779651964870759629195088099828group.public}",
  "{  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,  microcredits: 23437500000000u64.private,  _nonce: 4536868268814456227312360347031739423312689137706933033938812386306238998060group.public}",
  "{  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,  microcredits: 23437500000000u64.private,  _nonce: 205967862164714901379497326815256981526025583494109091059194305832749867953group.public}",
  "{  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,  microcredits: 23437500000000u64.private,  _nonce: 4424806931746512507605174575961455750579179367541686805196254590136284583805group.public}"
]
```

Setting `block_number` to `1` or higher will exclude the above credit records from the scan.
