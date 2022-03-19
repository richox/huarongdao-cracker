mod state;
mod search;

use state::State;

#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "huarongdao-cracker")]
struct Opt {
    input: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = structopt::StructOpt::from_args();
    let input_state_str = std::fs::read_to_string(&opt.input)?;

    let state = State::try_from(input_state_str)?;
    match search::search(&state, 3, 1) {
        Some(solved_state_seq) => {
            demo(&solved_state_seq);
        }
        None => {
            println!("走投无路！");
        }
    }
    Ok(())
}

fn demo(solved_state_seq: &[State]) {
    for (step, solved_state) in solved_state_seq.iter().enumerate() {
        println!("\x1b[H\x1b[2J"); // clear!
        println!("step: {}", step);
        println!("==========");
        println!("{}", solved_state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
