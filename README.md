# Cronny

**Cronny** is a pure-Rust cross-platform foreground Cron Job handler.
It's a foreground program that handles cron jobs.

## Usage
Cronny takes one parameter - the file path.
For example:
```sh
$ cronny file.cron
```

## Syntax
The Cronny syntax is split into two parts:
- the CRON Expression
- the command expression
These parts are split with `::`.

### Example
```crny
* * * * * * :: /path/to/script.sh
```
You can also use literal commands:
```crny
* * * * * * :: touch $(date).done
```
Please note that while it's possible to run it from anywhere and use relative paths, it's recommended to keep the path absolute (and if possible, user-agnostic, such as don't use `$HOME` etc.)

## FAQs

### Why not a background service?
Different platforms have different ways of background services. The best option is to use the platform's process handler, such as **SystemD**/**OpenRC** in Linux, **LaunchD** in macOS, or the equivalent in Windows (if there's any).

Alternatively, you can also use the `&` syntax on POSIX-compliant shells, such as:
```sh
$ cronny file.cron &
```
This will keep it running in the background.

### Does it log?
Yes, unlike most CRON Job Handlers, Cronny *does* log it. You can, however, disable logging entirely, by setting the `CRNY_LOG` to `0`.

### Is it better than \[insert name here\]?
Probably. Probably not. It's meant as a personal project, and is not meant to be used in production.

### Why Rust?
Why not?

## License
Cronny is licensed under [**Apache 2.0**](./LICENSE) license.
