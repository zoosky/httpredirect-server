+++
template = "docs.html"
insert_anchor_links = "right"
+++

# Getting started

To use httpredirectserver in your Rust projects, simply add it to your `Cargo.toml`:

```toml
httpredirectserver = "0.1"
```




And add the following to your `lib.rs` or `main.rs` if you are not using Rust 2018:

```rs
extern crate httpredirectserver;
```

You can view everything httpredirectserver exports on the [API docs](https://docs.rs/httpredirectserver).

# Usage

The primary method of using httpredirectserver is to load and parse all the redirect definitins in a given glob.

Let's take the following directory as example.

```sh
todo
```

Assuming the Rust file is at the same level as the `templates` folder, we can get a httpredirectserver instance that way:

```rs
use httpredirectserver::httpredirectserver;

// Use globbing
let httpredirectserver = match httpredirectserver::new("templates/**/*.csv") {
    Ok(t) => t,
    Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
    }
};
```

Compiling templates is a step that is meant to only happen once: use something like [lazy_static](https://crates.io/crates/lazy_static)
to define a constant instance.


## Advanced usage


