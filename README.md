# Watch306

A small Discord bot for monitoring CS306 @ [acmCSUF](https://acmcsuf.com)!

## Installation

You need the OpenSSL headers and a Rust toolchain >= 1.74. 
You may also want to install tmux to run the instance in.

Download the repository:

```sh
git clone https://github.com/amyipdev/watch306
```

## Usage

Watch306 will poll https://cs306.acmcsuf.com once every 10 seconds. (If you want to apply this to a different
server, you'll need to edit the sources and change the link; envvar-based links will be added on request).

You need to set up a Discord bot and invite it to the server, then get the Guild ID. Watch306 will only bind
to channels named `acm-server`; if you want to change this, either edit the code manually, or submit an issue
asking for the change (I will gladly add it).

Once added to the server and running, simply run `306!bind <MESSAGE>`. You can ping people in the setup message
to have them pinged on server state changes.

## Limitations

All the limitations mentioned in Usage, which will be corrected upon request.
