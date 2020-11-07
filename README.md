# (WIP) ipmap

A program which maps the ip addresses of the servers you connect to onto a world map in real time.

## Building
```
cargo build --release
```

Note: you must have privileges to capture using the pcap API. In order to give the binary the necessary permissions, run (for Linux):
```
sudo setcap cap_net_raw,cap_net_admin=eip <path to ipmap binary>
```

