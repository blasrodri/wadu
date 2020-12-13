use wadu::WaduError;

fn main() -> Result<(), WaduError> {
    wadu::start_cli().map(|_| ()) // for now, do not process the result.
}
