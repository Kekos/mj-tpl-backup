# mj-tpl-backup

A backup tool for Mailjet templates, written in Rust.

## Configuration

This tool can be configured with multiple Mailjet accounts. If you only have one account,
a "default" account is assumed in all commands.

Add a new account:

```
./mj-tpl-backup config new
```

Answer the three questions.

**NOTE:** Your private Mailjet API key will be stored in *cleartext* at `~/.mj-tpl-backup`.

You can list all configured accounts with:

```
./mj-tpl-backup config list
```

## Usage

To make a backup of the "default" configured account:

```
./mj-tpl-backup backup
```

Backups are stored at `~/mj-tpl-backups/<account name>` with each template stored in a
`.html` file and `.txt` file named by the template's ID.

To select account to back up:

```
./mj-tpl-backup backup <account name>
```

## License

MIT
