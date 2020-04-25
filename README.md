# ttrack

Command-line utility for time tracking and reporting.

## Features

1. Start time tracking
2. Stop time tracking
3. Edit tracked activities
4. Remove tracked activities
5. Tagging
6. Reporting

## Installation

```
cargo install ttrack
```

## Usage

For showing help:

```
$ ttrack help
$ ttrack <command> --help
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

Output format:

```
<ID> <START_TIME> - <END_TIME> <DURATION> <ACTIVITY_NAME> <TAGS>
```

The tool can also generate a report that contains time spent broken down by tags:

```
$ ttrack report
00:33:42 project:ttrack
00:32:28 client:personal
00:01:19 untagged
00:00:14 client:Big Co
```

## Commands

### Start Tracking

```
$ ttrack start --help
Start tracking an activity

USAGE:
    ttrack start [OPTIONS] <ACTIVITY_NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tag <tags>...    Activity tags. You can supply multiple values for this flag

ARGS:
    <ACTIVITY_NAME>    Activity name
```

### Stop Tracking

```
$ ttrack stop --help
Finish tracking current activity

USAGE:
    ttrack stop

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

### Edit Tracked Activity

```
$ ttrack edit --help
Edit tracked activity

USAGE:
    ttrack edit [OPTIONS] <ACTIVITY_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --end-time <end-time>             Edit the activity end time
    -n, --name <name>                     Edit the activity name
    -s, --start-time <start-time>         Edit the activity start time
    -a, --tags-to-add <tags-to-add>...    Tags to be added. You can supply multiple values for this tag
    -d, --tags-to-del <tags-to-del>...    Tags to be removed. You can supply multiple values for this tag

ARGS:
    <ACTIVITY_ID>    ID of the activity to be changed. e.g. `6aed4521`. You can get IDs from running `history`
                     command
```

### Show History

```
$ ttrack history --help
Show the list of tracked activities, starting from recent ones

USAGE:
    ttrack history [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --end-time <end-time>         [default: 3000-01-01T00:00:00+00:00]
    -p, --page <page>                 [default: 0]
    -n, --page-size <page-size>       [default: 20]
    -s, --start-time <start-time>     [default: 1970-01-01T00:00:00+00:00]
```

### Report

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
```

## Exporting Data

The application data is stored in these folders:

|Platform | Value                                    | Example                                  |
| ------- | ---------------------------------------- | ---------------------------------------- |
| Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/ttrack/ttrack.db | /home/alice/.local/share/ttrack/ttrack.db                 |
| macOS   | `$HOME`/Library/Application\ Support/ttrack/ttrack.db      | /Users/Alice/Library/Application\ Support/ttrack/ttrack.db |
| Windows | `{FOLDERID_LocalAppData}`\ttrack\ttrack.db                | C:\Users\Alice\AppData\Local\ttrack\ttrack.db            |

`ttrack.db` is a SQLite file.