# ttrack

Command-line utility for reporting & tracking time spent on activities.

## Installation

```
cargo install ttrack
```

## Usage

For showing help:

```
$ ttrack help
$ ttrack <subcommand> --help
```

Example usage:

```
$ ttrack start "Setup ubuntu server" -t client:BigCo -t "project:BigCo Homepage"
edfb7ef7 2020-04-25T15:00:06+08:00 - ......................... 00:00:00 Setup ubuntu server............................... [client:BigCo] [project:BigCo Homepage]

$ ttrack stop
edfb7ef7 2020-04-25T15:00:06+08:00 - 2020-04-25T15:01:14+08:00 00:01:08 Setup ubuntu server............................... [client:BigCo] [project:BigCo Homepage]

$ ttrack start "Setup CI pipeline" -t client:BigCo -t "project:BigCo Homepage"
e09c79d4 2020-04-25T15:03:30+08:00 - ......................... 00:00:00 Setup CI pipeline................................. [client:BigCo] [project:BigCo Homepage]

$ ttrack history
e09c79d4 2020-04-25T15:03:30+08:00 - ......................... 00:00:33 Setup CI pipeline................................. [client:BigCo] [project:BigCo Homepage]
edfb7ef7 2020-04-25T15:00:06+08:00 - 2020-04-25T15:01:14+08:00 00:01:08 Setup ubuntu server............................... [client:BigCo] [project:BigCo Homepage]

$ ttrack edit e09c79d4 -n "Setup CI/CD pipeline" -d client:BigCo -a project:ttrack -a client:myself
e09c79d4 2020-04-25T15:03:30+08:00 - ......................... 00:00:33 Setup CI/CD pipeline.............................. [client:myself] [project:BigCo Homepage] [project:ttrack]
```

The tool can also generate a report that contains time spent broken down by tags:

```
$ ttrack report --help
Show report of total activities duration broken down by tag

USAGE:
    ttrack report [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --end-time <end-time>         [default: 3000-01-01T00:00:00+00:00]
    -s, --start-time <start-time>     [default: 1970-01-01T00:00:00+00:00]

$ ttrack report -s 2020-04-21T00:00:00+08:00
00:33:42 project:ttrack
00:32:28 client:personal
00:01:19 untagged
00:00:14 client:Big Co
```

## Exporting Data

The application data is stored in these folders:

|Platform | Value                                    | Example                                  |
| ------- | ---------------------------------------- | ---------------------------------------- |
| Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/ttrack/ttrack.db | /home/alice/.local/share/ttrack/ttrack.db                 |
| macOS   | `$HOME`/Library/Application\ Support/ttrack/ttrack.db      | /Users/Alice/Library/Application\ Support/ttrack/ttrack.db |
| Windows | `{FOLDERID_LocalAppData}`\ttrack\ttrack.db                | C:\Users\Alice\AppData\Local\ttrack\ttrack.db            |

`ttrack.db` is a SQLite file.