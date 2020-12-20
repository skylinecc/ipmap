% IPMAP(1) ipmap 0.1.6

# NAME
ipmap - show connected IP addresses

# SYNOPSIS
**ipmap** [*FLAGS*]\ [*OPTIONS*]

# DESCRIPTION
ipmap is an interactive map that shows connected IP addresses. Simply launch it and go to localhost:700 to see a live map of what IP addresses you have connected to.

# FLAGS
**-h**, **--help**
: Prints help information.

**--headless**
: Launches the program without running the webserver.

**-V**, **--version**
: Prints version information.

# OPTIONS

**-p**, **--port**
: Set webserver port to launch on, if not set it defaults to port 700.

**-s**, **--service**
: Choose Geolocation API. Possible values: ipwhois, ipapi, ipapico, freegeoip.

**-w**, **--write-to-file**
: Set a path to write JSON to.

# AUTHORS
Copyright 2020 Skyline Coding Club Authors under the GPL-3 license.

Aditya Suresh <ethanaditya@gmail.com>

Grant H. <grantshandy@gmail.com>

Nick Z. <nicholasz2510@gmail.com>

sigmaSd
