use clap::Parser;
use term_size;

mod automata;


#[derive(Parser)]
struct Args {
    /// Delay (milisec)
    #[clap(short, long, default_value_t = 500)]
    delay: u64,

    /// Birth rule
    #[clap(short, long, default_value = "3")]
    birth: String,

    /// Survive rule
    #[clap(short, long, default_value = "23")]
    survive: String,

    /// Initial ratio (%)
    #[clap(short, long, default_value_t = 50)]
    ratio: u32,

    /// Dot color (0-255)
    #[clap(short, long, default_value_t = 198)]
    color: u8,
}

fn main() {
    let args = Args::parse();

    let delay = std::time::Duration::from_millis(args.delay);

    let (width, height) = term_size::dimensions().expect("Could not determine terminal size!");

    let birth = automata::rule_from_string(&args.birth);
    let survive = automata::rule_from_string(&args.survive);

    println!("{:?}", birth);
    println!("{:?}", survive);

    let mut board = automata::Board::new(width ,height);
    board.randomize(args.ratio, 100);

    let mut terminal = automata::Terminal::new(width, height, args.color);
    automata::clear_terminal();

    ctrlc::set_handler(move || {
        automata::close_terminal(height);
        std::process::exit(0);
    }).unwrap();

    loop {
        terminal.print_tiles(&board.tiles);
        automata::automata_step(&mut board, birth, survive);
        std::thread::sleep(delay);
    }
}
