use chrono::Local;

pub fn banner() {
    println!(r#"
 ██████╗ ██████╗ ███████╗███╗   ██╗██████╗ ██████╗ ██╗███╗   ███╗███████╗
██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔══██╗██║████╗ ████║██╔════╝
██║   ██║██████╔╝█████╗  ██╔██╗ ██║██████╔╝██████╔╝██║██╔████╔██║█████╗
██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║██╔═══╝ ██╔══██╗██║██║╚██╔╝██║██╔══╝
╚██████╔╝██║     ███████╗██║ ╚████║██║     ██║  ██║██║██║ ╚═╝ ██║███████╗
 ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝

  Open. Prime. Unstoppable.  v{}
"#, env!("CARGO_PKG_VERSION"));
}

pub fn info(msg: &str)    { println!("  \x1b[34m●\x1b[0m  {}", msg); }
pub fn success(msg: &str) { println!("  \x1b[32m✓\x1b[0m  {}", msg); }
pub fn warn(msg: &str)    { println!("  \x1b[33m⚠\x1b[0m  {}", msg); }
pub fn error(msg: &str)   { eprintln!("  \x1b[31m✗\x1b[0m  {}", msg); }
pub fn prompt()           { print!("\x1b[36mprime>\x1b[0m "); }
pub fn dim(msg: &str)     { println!("  \x1b[2m{}\x1b[0m", msg); }
