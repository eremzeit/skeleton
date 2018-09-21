use uci::timer::{UciTimer};

pub struct EngineSettings { }
impl EngineSettings {
    pub fn default() -> EngineSettings {
        EngineSettings {

        }
    }
}



pub struct SearchEngine {
    pub orig_board: Board,
    pub timer: UciTimer,
    settings: EngineSettings,
    //table: Table,
    //killers: Vec<Killer>,
    //rep: Vec<Hash>,
    ply: usize,
    node_count: usize,
    //irreversible: usize
}

impl SearchEngine {
    /// Create a new searcher from the start position
    pub fn new(settings: EngineSettings, timer: Timer) -> Self {
        let start = Board::starting_position();

        SearchEngine {
            orig_board: start,
            timer: timer,
            settings: settings,
            //table: Table::empty(settings.table_size),
            //killers: vec![Killer::EMPTY],
            //rep: vec![start.hash],
            ply: 0,
            node_count: 0,
            //irreversible: 0
        }
    }
    

    pub fn uci_update_settings(params: &Params) {

        while let Some(_) = params.find(|&word| word == "name") {
            let setting: &str = &params.next().unwrap().to_lowercase();

            match setting {
                // "hash" => {
                //     let size_mb = parse(params.nth(1));
                //     self.table = Table::empty_mb(size_mb);
                //     self.settings.table_size = self.table.size();
                // },
                _ => ()
            }
        }
    }

    // drop any caching and reset the board
    pub fn reset() {

    }
    
    // Is used to update the searcher with the position that the UI prefers.  
    pub fn position(to_fen: Option<String>, moves: &Vec<Move>) {
        self.orig_board = match (to_fen) {
            None       => Board::start_position(),
            _fen       => Board::from_fen(fen),
        };

        self.node_count = 0;
        // self.rep = vec![self.root.hash];
        // self.killers = vec![];
    
        // // Remove half move, full move, and other words until there are moves
        // for mv_str in params.skip_while(|&val| val != "moves").skip(1) {
        //     let mv = self.root.move_from_str(mv_str);
        //     if self.root.is_irreversible(mv) {
        //         self.irreversible = self.root.ply + 1;
        //     }
        //     self.root.make_move(mv);
        //     self.rep.push(self.root.hash);
        // }
    };
}
