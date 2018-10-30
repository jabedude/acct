# Rust acct - parse acct(5) files

[![Crates.io](https://img.shields.io/crates/v/acct.svg)](https://crates.io/crates/acct)

A library for handling UNIX process accounting files.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
acct = "0.5.0"
```

## Example
```rust
    let mut file = File::open(acct_file).unwrap();

    let acct_file = AcctFile::new(&mut file).unwrap();
    for acct in &acct_file.records {
        let datetime = DateTime::<Utc>::from(acct.creation_time);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        println!("{}\t{}\t{:?}\tSU:{}", acct.command, acct.username, timestamp_str, acct.was_super_user());
    }

    let mut out = File::create("optfile").unwrap();
    let bytes = acct_file.into_bytes().unwrap();
    out.write_all(&bytes);
```

## Documentation

[acct reference](https://docs.rs/acct)
