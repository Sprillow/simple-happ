


You will want this version of holochain_cli installed:
```
cargo install holochain_cli --version 0.2.0 --locked
```

Depending on whether you are trying to build it for iOS, or run it on desktop, you should 
1. want to build for iOS: the properties `dylib` in dna/workdir/dna.yaml should NOT be commented
2. want to build for desktop: the properties `dylib` in dna/workdir/dna.yaml SHOULD be commented