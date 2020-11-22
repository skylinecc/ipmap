<h1 align="center">ipmap</h1>

<p align="center">An interactive map that shows connected IP addresses.</p>


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
# target/release/ipmap
```

**To use it navigate to your web browser and go to `localhost:8000`, where the map will appear**

## Command Line Options
```
FLAGS:
    -h, --help        Prints help information
        --headless    Launches the program without opening the browser
    -V, --version     Prints version information
```
