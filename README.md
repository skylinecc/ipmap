<h1 align="center">ipmap</h1>

<p align="center">An interactive map that shows connected IP addresses.</p>
<p align="center"><img src=data/screenshot.png></p>

## Requirements 
ipmap uses `libpcap-dev`, which is only easily available on UNIX-like systems (Linux, MacOS, *BSD).

Windows is possible, but [WinPcap](https://github.com/ebfull/pcap#windows) is required before build time.

## Building
Because this program is written in rust, you must have rust [installed](https://www.rust-lang.org/tools/install).

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

**To use it navigate to your web browser and go to `localhost:700`, where the map will appear.**

## Services
The IP geolocation service used in ipmap can be changed at the start using the command line flag.

Each service included in this library has a weekly, hourly, or monthly limit.
Some have more free queries, but are less reliable.

Here are the query limits:

| Service       | Limit                     |
| ---------     | ------------------------- |
| ipwhois       | 10,000/month              |
| freegeoip     | 15,000/hour               |
| ipapi         | 45/minute                 |
| ipapico       | 1,000/day (30,000/month)  |

If no service specified, ipapi will be used, which will limit how many IP is detected per minute.

## Command Line Options
```
ipmap 0.1.5
Skyline High Coding Club Authors

USAGE:
    ipmap [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
        --headless    Launches the program without running the webserver
    -V, --version     Prints version information

OPTIONS:
    -p, --port <PORT>             Set webserver port, default port 700
    -s, --service <SERVICE>       Choose Geolocation API [possible values: ipwhois, ipapi, ipapico, freegeoip]
    -w, --write-to-file <PATH>    Set path to write JSON to
```

## Creating Debian Packages
Creating debian packages is fairly easy, as I've created a makefile for it.

Dependencies:
```
# apt install build-essential devscripts debhelper libpcap-dev make
```

Then just type:
```
make deb-gen
```
