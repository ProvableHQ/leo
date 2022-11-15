# Troubleshooting Guide

In this guide, we will cover some common issues that may arise when installing and using Leo for the first time.

## Downloading parameters

When running `leo run` for the first time, Leo will download the necessary parameters from a remote server.
This may take a while and may outright fail, depending on your internet connection and network configuration.

You will know that the download has failed if you see the following error message or something similar:

```bash
ATTENTION - "genesis.prover.1c9bbe9" does not exist, downloading this file remotely and storing it locally. Please ensure "genesis.prover.1c9bbe9" is stored in "/Users/xxx/.aleo/resources/genesis.prover.1c9bbe9".

snarkvm_parameters::testnet3 - Downloading parameters...
snarkvm_parameters::testnet3 - thread `main` panicked at 'Failed to load proving key: Crate("curl::error", "Error { description: \"Transferred a partial file\", code: 18, extra: Some(\"transfer closed with 92197356 bytes remaining to read\") }")', /Users/xxx/.cargo/git/checkouts/snarkvm-f1160780ffe17de8/ea14990/parameters/src/testnet3/symbol_table_creation:95:9
stack backtrace: 
   0: backtrace::capture::Backtrace::new
   1: leo::set_panic_hook::{{closure}}
   2: std::panicking::rust_panic_with_hook
   3: std::panicking::begin_panic_handler::{{closure}}
   4: std::sys_common::backtrace::__rust_end_short_backtrace
   5: _rust_begin_unwind
   6: core::panicking::panic_fmt
   7: core::result::unwrap_failed
   8: std::sync::once::Once::call_once::{{closure}}
   9: std::sync::once::Once::call_inner
  10: snarkvm_compiler::process::Process<N>::load
  11: snarkvm::package::Package<N>::get_process
  12: snarkvm::package::build::<impl snarkvm::package::Package<N>>::build
  13: aleo::commands::build::Build::parse
  14: <leo::commands::build::Build as leo::commands::Command>::apply
  15: leo::commands::Command::execute
  16: leo::commands::Command::try_execute
  17: leo::run_with_args
  18: scoped_tls::ScopedKey<T>::set
  19: leo::main
  20: std::sys_common::backtrace::__rust_begin_short_backtrace
  21: std::rt::lang_start::{{closure}}
  22: std::rt::lang_start_internal
  23: _main
```

If this happens, try using the following script to download the parameters until it succeeds:

```bash
#!/bin/bash
echo "
Downloading parameters. This will take a few minutes...
"

# Create a new Leo project.
leo new install > /dev/null 2>&1 
cd install 

# Attempt to compile the program until it passes.
# This is necessary to ensure that the universal parameters are downloaded.
declare -i DONE

DONE=1

while [ $DONE -ne 0 ]
do
  leo build 2>&1
  DONE=$?
  sleep 0.5
done

# Remove the program.
cd .. && rm -rf install
```
