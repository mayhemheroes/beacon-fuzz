# beaconfuzz_v2

This tool help to find logic bug using differential fuzzing accross multiple eth2 client implementation.

## Setup and Installation

After setup and installation, your workspace should look as following:
```
├── beacon-fuzz
├── lighthouse
├── nimbus-eth2
├── prysm
└── teku
```

### Beaconfuzz_v2 setup

Clone this repository
``` sh
git clone https://github.com/sigp/beacon-fuzz
```

### lighthouse setup

Clone the repository of lighthouse:
``` sh
git clone https://github.com/sigp/lighthouse
```

### nimbus setup

Install nimbus dependencies:
``` sh
sudo apt install libpcre3-dev
```

Clone the repository of nimbus and compile the nimbus fuzzing library:
``` sh
git clone https://github.com/status-im/nimbus-eth2
cd nimbus-eth2
git checkout devel
NIMFLAGS="-d:disableLTO" make libnfuzz.a
```

Finally, set the following variable with the current path of nimbus:
``` sh
export CARGO_NIMBUS_DIR=~/path/to/nimbus-eth2
```

### prysm setup
<!---
Create a prysm folder:
```
mkdir prysm
cp -r beacon-fuzz/beaconfuzz_v2/libs/pfuzz prysm/
```

Compile the prysm fuzzing library:
```
go get .
go build -o libpfuzz.a -tags=blst_enabled,libfuzzer -buildmode=c-archive pfuzz.go
```
 -->
Set the following variable with the current path of prysm:
``` sh
export CARGO_PRYSM_DIR=beacon-fuzz/beaconfuzz_v2/libs
```

### teku setup

Install teku dependencies:
``` sh
# Install Java 11 or greater
sudo apt install openjdk-11-jdk clang
```

Setup `$JAVA_HOME`:
``` sh
# Ensure `JAVA_HOME` is set.
echo $JAVA_HOME
# (If `echo $JAVA_HOME` is displays no output) it should probably be set to something like:
export JAVA_HOME="$(dirname $(dirname $(readlink -f $(command -v java))))"
```

Probably want to add it to your `.profile`
(This is `/usr/lib/jvm/java-11-openjdk-amd64` in ubuntu)

Add `$JAVA_HOME/lib/server` to your runtime library path via *either* of the following methods:

**via LD_LIBRARY_PATH**

``` sh
export LD_LIBRARY_PATH="$JAVA_HOME/lib/server"
```

This needs to be set at runtime - i.e. whenever you want to run the teku fuzzer, not when you're building it.

Or

**via ldconfig**

``` sh
echo "$JAVA_HOME/lib/server" >> /etc/ld.so.conf.d/java.conf
sudo ldconfig
```

<!--
Also adding this?
$ echo "$JAVA_HOME/lib" >> /etc/ld.so.conf.d/java.conf
-->


Clone teku repository:
``` sh
git clone --branch 0.12.9 https://github.com/PegaSysEng/teku.git
```

Set `BFUZZ_TEKU_DIR` to the root teku directory:
``` sh
cd path/to/teku
BFUZZ_TEKU_DIR="$(realpath -e .)" && export BFUZZ_TEKU_DIR
```

Build teku:
```sh
cd teku
./gradlew installDist fuzz:build -x test --stacktrace
```


### Beaconfuzz_v2 compilation

Compile the project using the Makefile
``` sh
cd beacon-fuzz/beaconfuzz_v2
make
```

Install rust fuzzers:
``` sh
cargo +nightly install cargo-fuzz
cargo +nightly install honggfuzz
```

Compile and run the fuzzers:
``` sh
make fuzz_*
fuzz_attestation               fuzz_block                     fuzz_proposer_slashing
fuzz_attestation-struct        fuzz_block-struct              fuzz_proposer_slashing-struct
fuzz_attester_slashing         fuzz_deposit                   fuzz_voluntary_exit
fuzz_attester_slashing-struct  fuzz_deposit-struct            fuzz_voluntary_exit-struct
```

there is two differents kind of fuzzing targets:
- `fuzz_*`: Mutation fuzzing using honggfuzz
- `fuzz_*-struct`: structural fuzzing using libfuzzer + arbitrary

<!---
RUSTFLAGS='-L /home/scop/Documents/consulting/sigmaprime/prysm/pfuzz/ -L /home/scop/Documents/consulting/sigmaprime/nim-beacon-state/build/ ' make fuzz_block-struct
 -->


## Other Useful Troubleshooting/tips

- You can pass libfuzzer flags using `cargo fuzz` like `cargo fuzz run -- -help=1`
  - Useful: `-rss_limit_mb=0` to disable memory usage limits
- You can pass extra Java parameters via the `JAVA_TOOL_OPTIONS` env var
  - This can be anything that you'd normally add as a flag when running a java program i.e. `java -XX:DumpLoadedClassList=hello.classlist hello.jar`
  - e.g. `export JAVA_TOOL_OPTIONS='-Xmx6g -Xcheck:jni'`
    to set the max heap size for the JVM to 6GiB and do extra JNI parameter checking.
