mod timer;

use timer::{UciTimer, TimeSettings};

//use std::io::prelude::*;
use std::io::{stdin, BufReader};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use time;

use type

const ENGINE_NAME: &'static str = "Skeleton 0.0.1";

pub fn main_loop() {
    let should_stop = Arc::new(AtomicBool::new(false));
    let timer = Timer::default(should_stop.clone());

    let searcher = Arc::new(Mutex::new(Searcher::new(EngineSettings::default(), timer)));

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or("".into());
        let mut params: Params = line.split_whitespace();

        if let Some(first_word) = params.next() {
            match first_word {
                "uci"        => uci(),
                "isready"    => println!("readyok"),
                "setoption"  => lock!(searcher).uci_update_settings(&mut params),
                "ucinewgame" => lock!(searcher).reset(),
                "position"   => lock!(searcher).unwind_to_position(&mut params),

        // // Remove half move, full move, and other words until there are moves
        // for mv_str in params.skip_while(|&val| val != "moves").skip(1) {
        //     let mv = self.root.move_from_str(mv_str);
        //     if self.root.is_irreversible(mv) {
        //         self.irreversible = self.root.ply + 1;
        //     }
        //     self.root.make_move(mv);
        //     self.rep.push(self.root.hash);
        // }
                "stop"       => should_stop.store(true, Ordering::Relaxed),
                "quit"       => return,
                //"perft"      => perft(&lock!(searcher).root, &mut params),
                //"test"       => run_test(&mut lock!(searcher), params.next()),
                "go"         => {
                    lock!(searcher).timer.replace(&mut params);

                    let searcher = searcher.clone();
                    thread::spawn(move || {
                        lock!(searcher).go();
                    });
                },
                _ => println!("Unknown command: {}", first_word)
            }
        }
    }
}

pub fn position(&mut params: Params) {
    // Remove half move, full move, and other words until there are moves
    for mv_str in params.skip_while(|&val| val != "moves").skip(1) {
        //Move.
        let mv = self.root.move_from_str(mv_str);
        if self.root.is_irreversible(mv) {
            self.irreversible = self.root.ply + 1;
        }
        self.root.make_move(mv);
        self.rep.push(self.root.hash);
    }
}

#[derive(Copy, Clone)]
pub struct EngineSettings {
    pub table_size: usize
}

impl Default for EngineSettings {
    fn default() -> Self {
        EngineSettings {
            //table_size: 10_000_000
        }
    }
}

// pub fn run_test(searcher: &mut Searcher, test: Option<&str>) {
//     match test {
//         Some("perf") => positions("testing/positions/performance",
//                 searcher, &mut |s| s.go()),
//         Some("move") => positions("testing/positions/perftsuite.epd",
//                 searcher, &mut |s| println!("{}", s.root.perft(6, true))),
//         _ => println!("Error: Valid options are `perf` or `move`")
//     };
// }

// pub fn perft(board: &Board, params: &mut Params) {
//     let depth = parse_or(params.next(), 5);
// 
//     println!("total = {}\n", board.perft(depth, true));
// }

// 
// pub fn positions(path: &str, searcher: &mut Searcher, do_work: &mut FnMut(&mut Searcher)) {
//     let file = match File::open(path) {
//         Ok(file) => BufReader::new(file),
//         Err(e)   => panic!("Test suite {} could not be read. {:?}", path, e)
//     };
// 
//     let start = time::precise_time_s();
//     searcher.timer.replace(&mut "wtime 10000 btime 10000 movestogo 1".split_whitespace());
// 
//     for line in file.lines().take(10) {
//         let fen = String::from("fen ") + &line.unwrap();
//         println!("{}", fen);
// 
//         searcher.position(&mut fen.split_whitespace());
//         do_work(searcher);
//     }
//     println!("Time taken = {} seconds", time::precise_time_s() - start);
// }

pub fn uci() {
    println!("id name {}", ENGINE_NAME);
    println!("id author Alan Jones");
    println!("option name Hash type spin min 1 max {} default 128", 1024 * 1024);
    println!("uciok");
}
