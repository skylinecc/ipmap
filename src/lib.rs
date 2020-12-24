use clap::{crate_version, App, Arg};

pub fn init_app () -> App<'static, 'static> {
    App::new("ipmap")
        .version(crate_version!())
        .author("Skyline High Coding Club Authors")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without running the webserver")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("service")
                .long("service")
                .short("s")
                .help("Choose Geolocation API, if not set it defaults to ipapi")
                .required(false)
                .takes_value(true)
                .value_name("SERVICE")
                .possible_value("ipwhois")
                .possible_value("ipapi")
                .possible_value("ipapico")
                .possible_value("freegeoip"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .help("Set webserver port to launch on, if not set it defaults to port 700")
                .required(false)
                .takes_value(true)
                .value_name("PORT"),
        )
        .arg(
            Arg::with_name("write-to-file")
                .long("write-to-file")
                .short("w")
                .help("Set a path to write JSON to")
                .required(false)
                .takes_value(true)
                .value_name("PATH"),
        )
}