fn main() {
    println!("Hello, world!");
    send_command();
}

fn send_command() -> anyhow::Result<()> {
    std::process::Command::new("pmset")
        .arg("displaysleepnow")
        .spawn()?;

    Ok(())
}
