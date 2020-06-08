# keyrate

Configure keyboard repeat rate and delay time for Windows 10.

# Motivation

- Wanted to speed up VIM ijkl movement and other repetitive actions.
- Needed an accessible alternative to editing the Windows Registry keyboard
  settings.

# Usage

```txt
keyrate 0.1.0

Configure keyboard repeat rate and delay time for Windows 10.

This program uses Filter Keys to pseudo modify keyboard settings. If no
arguments are present, the program disables Filter Keys. Otherwise, the
program enables and initializes Filter Keys with the given or
defaulted parameters.

USAGE:
    keyrate.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <delay>        Delay time (250ms)
    -r <rate>         Clicks per second (31)
```

