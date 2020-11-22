<h1 align="center">ipmap</h1>

<p align="center">A program which maps the ip addresses of the servers you connect to onto a world map in real time.</p>


<p align="center" class="aligncenter">
    <img src=https://github.com/skylinecc/ipmap/blob/main/data/screenshot.png>
</p>

## Requirements 
ipmap uses `libpcap`, which is only easily available on UNIX-like systems (macOS, Linux, FreeBSD, OpenBSD, NetBSD).
## Building
Because this program is written in rust, you must have cargo [installed](https://www.rust-lang.org/tools/install).

First, build it:
```
$ cargo build --release
```

You must have privileges to capture using the pcap API. In order to give the binary the necessary permissions, run:
```
# setcap cap_net_raw,cap_net_admin=eip target/release/ipmap
```

Finally, execute it.
```
# ROCKET_PORT=<port> target/release/ipmap
```

## Command Line Options
```
FLAGS:
    -h, --help        Prints help information
        --headless    Launches the program without opening the browser
    -V, --version     Prints version information
```
