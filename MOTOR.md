# Motor OS support

This branch is based on upstream Wasmi 1.1.0 at
`8273dfb09d493971b7bb12fe614d740cdc857175`. It keeps all Wasmi crates in their
original workspace.

The changes preserve negative branch offsets when packing the IR comparator,
and add an owned, nonmoving linear-memory allocator interface. Javy provides
the Motor reservation implementation; Wasmi itself remains platform independent.
Default Vec and caller-owned static memory behavior and no-std support remain.

`./motor-test.sh` runs the IR regression, owned-memory lifetime/growth/error
regressions and no-std check with the adjacent Motor repository's selected
Rust toolchain. These tests belong here because they test this implementation.
There is no Motor-repository test dependency on a `/tmp` checkout.
