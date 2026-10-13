use std::process::Command;
mod config;

fn main() {
    let config = config::Config::new();

    dbg!(&config);

    println!("Hello, world!");

    println!("Hostname: {}", Machine::fetch_hostname());

    // send_command();
}

struct Machine {
    // hostname: String,
}

impl Machine {
    fn fetch_hostname() -> String {
        let hostname = Command::new("hostname")
            .output()
            .expect("Failed to set hostname");

        String::from_utf8_lossy(&hostname.stdout).trim().to_string()
    }

    fn send_sleep_command() -> anyhow::Result<()> {
        Command::new("pmset").arg("displaysleepnow").spawn()?;
        Ok(())
    }
}
