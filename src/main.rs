mod lib;

use clap::Parser;
use std::ffi::CStr;

#[derive(Parser, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    /// Show with hyphens
    #[clap(short, long, group = "format")]
    pub hyphenated: bool,

    /// Show as a urn
    #[clap(long, group = "format")]
    pub urn: bool,

    /// Show as uppercase (default: lowercase)
    #[clap(short, long)]
    pub uppercase: bool,
}

fn new(opts: Opts) -> String {
    let output_c_str =
        unsafe { CStr::from_ptr(lib::generate(opts.hyphenated, opts.urn, opts.uppercase)) };
    output_c_str.to_str().unwrap().to_string()
}

fn main() {
    let cli = Opts::parse();
    println!("{}", new(cli));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_uuid() {
        assert_eq!(new(Opts::default()).len(), 32);
    }
    #[test]
    fn test_hyphenated_uuid() {
        let mut opts = Opts::default();
        opts.hyphenated = true;
        assert_eq!(new(opts).len(), 36);
    }
    #[test]
    fn test_urn_uuid() {
        let mut opts = Opts::default();
        opts.urn = true;
        assert_eq!(new(opts).len(), 45);
    }
}
