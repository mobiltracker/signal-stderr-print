use std::io::Error;

#[cfg(unix)]
use async_std;
#[cfg(unix)]
use futures::stream::StreamExt;
#[cfg(unix)]
use signal_hook;
#[cfg(unix)]
use signal_hook_async_std::Signals;

#[cfg(unix)]
pub async fn handle_signals(signals: Signals) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            signal_hook::SIGHUP => {
                eprintln!("Received SIGHUP");
            }
            signal_hook::SIGTERM => {
                eprintln!("Received SIGTERM");
            }
            signal_hook::SIGINT => {
                eprintln!("Received SIGINT");
            }
            signal_hook::SIGQUIT => {
                eprintln!("Received SIGQUIT");
            }
            signal_hook::SIGUSR1 => {
                eprintln!("Received SIGUSR1");
            }
            _ => {
                eprintln!("unknown signal {:?}", signal);
            }
        }
    }
}

#[cfg(unix)]
pub fn spawn_signal_handler() -> Result<(), Error> {
    let signals = Signals::new(&[
        signal_hook::SIGHUP,
        signal_hook::SIGTERM,
        signal_hook::SIGINT,
        signal_hook::SIGQUIT,
        signal_hook::SIGUSR1,
    ])?;

    let _handle = signals.handle();

    let _signals_task = async_std::task::spawn(handle_signals(signals));
    println!("handling signals");
    Ok(())
}

#[cfg(windows)]
pub fn spawn_signal_handler() -> Result<(), Error> {
    println!("signal handling disabled for windows");
    Ok(())
}
