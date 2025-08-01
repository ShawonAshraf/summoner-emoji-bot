# summoner-emoji-bot

> A random emoji generator bot in Rust


[![Test](https://github.com/ShawonAshraf/summoner-emoji-bot/actions/workflows/test.yml/badge.svg)](https://github.com/ShawonAshraf/summoner-emoji-bot/actions/workflows/test.yml)

I have to approve a lot of pull requests, and I thought it would be fun to have a random emoji generator to use as an
approval message. Then I was suggested to make a discord bot based on it. There are two components to this project:

1. **Emoji Generator**: A Rust program that generates a random emoji from a predefined list and then copies it to the
   clipboard.
2. **Discord Bot**: A Discord bot that listens for a specific command and responds with a random emoji.

## Usage

> [!NOTE]
> Make sure to create a discord bot application on the discord developer portal with the following permission and scope first!:
> `bot` and `send messages`. Also enable `GUILD_MESSAGES` permissions in the previeleged access section so that the bot can read 
> instructions.

### Build

```bash
cargo build --release
```

### Run

```bash
# for the emoji generator
./target/release/summoner-emoji-bot
# for the discord bot
export DISCORD_TOKEN=your_token_here
./target/release/summoner-emoji-bot bot
```

To run the bot as a docker container:

```bash
docker build -t summoner-emoji-bot:latest .

docker run -e DISCORD_TOKEN=your_token_here summoner-emoji-bot:latest
```

### Building for Linux

> [!NOTE]
> This is in case you don't have access to a machine running linux

```bash
# make a directory at the project root named penguin to store the build
mkdir -p penguin
# Build the image
docker build -f penguin.Dockerfile -t summoner-linux-builder .

# Run the container with volume mount to save binaries locally
docker run -v ./:/bot -v ./penguin:/penguin summoner-linux-build
```
