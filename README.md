# Candid Export Bug

## Steps to reproduce

1. `git clone`
2. `cargo test`
3. Inspect the generated candid file at canisters/tests/test.did
4. Notice that it contains a type `ManualReply` which doesn't have the same name as the `User` struct in canisters/test/src/lib.rs
   ```candid
   type ManualReply = record { id : text };
   service : { method : () -> (ManualReply) query }
   ```
