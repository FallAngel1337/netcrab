use std::net::SocketAddr;

#[macro_export]
macro_rules! help {
    () => {
        {
            print!("{HELP_MENU}");
            std::process::exit(0);
        }
    };
}

const HELP_MENU: &str = "rustcat v0.1.0
Usage: rustcat [OPTION] [<addr:port>]
Options:
-h,  --help           Show this message
-l,  --listen         Listen mode\n";


#[derive(Debug, Clone, Copy)]
pub struct NetCrabArgs {
    pub addr: Option<SocketAddr>,
    pub listen: bool
}

impl NetCrabArgs {
    pub fn new() -> Self {
        let args = std::env::args().skip(1).take(2);
        if args.len() == 0 {
            help!()
        }
        
        let mut res = Self {
            addr: None,
            listen: false,
        };

        for arg in args {
            match arg.as_ref() {
                "-h" | "--help" => help!(),
                "-l" | "--listen" => res.listen = true,
                _ => res.addr = Some(arg.parse().unwrap()),
            }
        }

        res
    }
}