[Blazingly fast](./benches) 🚀 Twitch IRC parsing library written in Rust 🦀

This is a Twitch-specific IRC parser, it is not guaranteed to work for any other kind of IRC message.
It is also fairly low level, and doesn't provide a convenient high-level interface to build bots with.

```
$ cargo add --git https://github.com/jprochazk/twitch-rs.git
```

```rust
for message in data.lines().flat_map(twitch::parse) {
  if let (Some(prefix), Some(params)) = (message.prefix(), message.params()) {
    println!(
      "{}: {}",
      prefix.user.unwrap_or("<server>"),
      params.strip_prefix(':').unwrap_or(params),
    )
  };
}
```

If you're on x86, you can enable the `simd` feature for a ~50% increase in performance:
```
$ cargo add --git https://github.com/jprochazk/twitch-rs.git -F simd
```

## Acknowledgements

Initially based on [dank-twitch-irc](https://github.com/robotty/dank-twitch-irc), and [twitch-irc-rs](https://github.com/robotty/twitch-irc-rs).
