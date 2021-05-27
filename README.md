# file-loader-procedural

Rust library for storing encrypted content of any file at compile time in the binary.

Part of the **file-loader** lib.

Versions over **v1.0.0** are not designed to work separately from the **file-loader**.

## Example (under v1.0.0)
```Rust
#[macro_use] extern crate file_loader_procedural;

// Using 'strenc' crate to hide the encryption key
#[macro_use] extern crate strenc;
#[macro_use] extern crate magic_crypt;
strenc_initialize!();


fn main() {
    // Saving content of `Cargo.toml` with key `abvcd`
    let x = f_load!("Cargo.toml", enc!("abvcd"));

    // Printing encrypted content
    println!("'{}'", x);
}
```
