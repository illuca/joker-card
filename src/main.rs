use score_lib::{ Score, config, explain };

use std::{ error::Error, fs::File, io::{ Read, stdin }, path::{ Path, PathBuf } };

use clap::Parser;
use ortalib::{ Chips, Mult, Round };

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;
    config::init(opts.explain);

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round) -> (Chips, Mult) {
    explain!("{:?}", round);
    let mut score = Score::new(round);
    score.calculate_score();

    (score.chips, score.mult)
}
