mod uci_adapter;

use chess_kit::engine::Engine;
use uci_adapter::UciAdapter;

fn main() {
    if let Err(error) = run() {
        eprintln!("chess-kit: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::new()?;
    let mut adapter = UciAdapter::new(engine);
    chess_kit::comm::uci::run(&mut adapter)?;
    Ok(())
}
