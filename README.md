![ci badge]

# Magnolia

## Config

A `magnolia.cfg.yml` file, or whatever path is passed as argument the first argument, is required at the root of the
repository. This file contains the configuration for the bot.
The following is an example of the file structure:

```yaml
roles:
  devforum_member: "ROLE_ID"
  devforum_regular: "ROLE_ID"
```

[ci badge]:https://img.shields.io/github/actions/workflow/status/archasion/discord-bot-rs/ci.yml?branch=main&event=push&label=CI