use portable_pty::{CommandBuilder, PtyPair, PtySize, native_pty_system};

pub fn init_term() -> anyhow::Result<PtyPair> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;
    let cmd = CommandBuilder::new("bash");
    pair.slave.spawn_command(cmd)?;
    Ok(pair)
}

pub fn install_pkg(name: &str, pair: PtyPair) -> anyhow::Result<()> {
    let mut cmd = CommandBuilder::new("sudo");
    cmd.args(vec!["pacman", "-S", name]);
    pair.slave.spawn_command(cmd)?;
    Ok(())
}

pub fn remove_pkg(name: &str, pair: PtyPair) -> anyhow::Result<()> {
    let mut cmd = CommandBuilder::new("sudo");
    cmd.args(vec!["pacman", "-R", name]);
    pair.slave.spawn_command(cmd)?;
    Ok(())
}

pub fn send_command(command: &str, pair: PtyPair) -> anyhow::Result<()> {
    let cmd = CommandBuilder::new(command);
    pair.slave.spawn_command(cmd)?;
    Ok(())
}
