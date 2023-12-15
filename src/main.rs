use pulse::timer::Timer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut timers = vec![];

    timers.push(Timer::new("feeding cat"));
    timers.push(Timer::new("observing logs"));
    timers.push(Timer::new("shopping for coffee"));
    timers.push(Timer::new("typical programmer stuff"));

    let _ = timers[0].start();

    dbg!(timers);

    Ok(())
}
