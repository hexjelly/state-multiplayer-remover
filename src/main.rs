use elma::{state, ElmaError};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "state.dat multiplayer times remover")]
struct Opt {
    /// Input file
    #[structopt(long, short, parse(from_os_str), default_value = "state.dat")]
    input: PathBuf,

    /// Output file
    #[structopt(long, short, parse(from_os_str), default_value = "state_mp_removed.dat")]
    output: PathBuf,
}

fn main() -> Result<(), ElmaError> {
    let opt = Opt::from_args();
    let mut state = state::State::load(opt.input)?;

    for mut times in &mut state.times {
        times.multi = vec!();
    }

    state.save(opt.output)?;

    Ok(())
}
