# Wafflecord

Wafflecord is a discord bot that sends weekly notifications to subscribed channels. This bot was created when my friends from UC Berkeley wanted to start following the routine recommended by [this tiktok](https://www.tiktok.com/t/ZTYGguwgP/) so we could keep in touch after graduating. I felt it was the perfect opportunity to finally build something useful with Rust.

## Setup

Clone the repository and build with `cargo`:

```bash
git clone https://github.com/lucdar/wafflecord
cd wafflecord
cargo build --release
```

The bot requires two environment variables for operation:

* `WAFFLECORD_DISCORD_TOKEN`: The Discord bot's private token.
* `WAFFLECORD_SUBSCRIBERS_DIR`: A directory where the bot can store subscribed channels.

Once these environment variables are set, the bot can be run:

```bash
./target/release/wafflecord
```

## Acknowledgements

* This bot was built using Poise, a Rust Discord bot framework. [main.rs](src/main.rs) was modified from poise's [`basic_structure`](https://github.com/serenity-rs/poise/blob/current/examples/basic_structure/main.rs) example.

## License

[MIT](./LICENSE)
