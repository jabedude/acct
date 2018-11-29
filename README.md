![acct title](https://github.com/jabedude/acct/raw/master/acct.png)

# Rust acct - parse acct(5) files

[![Crates.io](https://img.shields.io/crates/v/acct.svg)](https://crates.io/crates/acct)

A library for handling UNIX process accounting files.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
acct = "0.6.0"
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
```

## Documentation

[acct reference](https://docs.rs/acct)
