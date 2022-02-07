use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod tile;
use tile::Tile;

pub struct Game {
    pub board: Vec<Tile>,
    pub width: u8,
    pub height: u8,
    pub solved: bool,
    pub move_count: u32,
}

impl Game {

    // Game actions

    pub fn click_tile(&mut self, index: u8) -> bool {
        log::info!("Clicked {:?}", self.board[index as usize]);

        if self.is_legal_move(index) && !self.solved {
            self.make_move(index);

            let order: Vec<u8> = self.board.iter().map(|tile| tile.id).clone().collect();
            self.solved = Game::is_solved(&order);
        }

        log::info!("Is legal move: {}", self.is_legal_move(index));

        true
    }

    pub fn restart(&mut self) -> bool {
        let new_order = Game::random_legal_order(self.width, self.height, true);

        let max_value = new_order.len() - 1;

        for i in 0..max_value {
            self.board[i] = Tile::new(new_order[i], i as u8 == max_value as u8)
        }

        self.solved = false;
        self.move_count = 0;

        true
    }

    // Initialize

    pub fn new(width: u8, height: u8) -> Game {
        let order = Game::random_legal_order(width, height, false);
        let mut board: Vec<Tile> = Vec::new();

        let max_value = order.len();
        for i in order {
            board.push(Tile::new(i, i == max_value as u8));
        }

        Game {
            board,
            width,
            height,
            solved: false,
            move_count: 0
        }
    }

    pub fn random_legal_order(width: u8, height: u8, real: bool) -> Vec<u8> {
        let mut order: Vec<u8> = Vec::new();

        for i in 1..(width * height + 1) {
            order.push(i);
        }

        if real {
        let mut rng = thread_rng();
        order.shuffle(&mut rng);

        while !Game::is_legal_order(&order, width, height) && !Game::is_solved(&order) {
            order.shuffle(&mut rng);
        }
    }

        order
    }

    fn is_legal_order(order: &Vec<u8>, width: u8, height: u8) -> bool {
        let mut inversions = 0;

        let mut blank_index = 0;

        for i in 0..order.len() {
            if order[i] as usize == order.len() {
                blank_index = i as u8;
                continue;
            }
            for j in i..order.len() {
                if order[i] > order[j] {
                    inversions += 1;
                }
            }
        }

        let blank_layer = height - ((blank_index / width) + 1);

        log::info!("Inversions {} | Blank layer {}", inversions, blank_layer);

        return (inversions % 2 == 0) != (blank_layer % 2 == 0);
    }

    fn is_solved(order: &Vec<u8>) -> bool {
        for (i, value) in order.iter().enumerate() {
            if &(i as u8 + 1) != value {
                return false;
            }
        }

        log::info!("Puzzle is solved!");
        true
    }

    fn is_legal_move(&self, index_clicked: u8) -> bool {
        let empty_pos = self.empty_index();

        if index_clicked == empty_pos {
            return false;
        }

        // TODO: Fix moves wrapping lines

        if index_clicked != 0 {
            let target = self.board.get(index_clicked as usize - 1);
            match target {
                Some(tile) => {
                    if tile.empty {
                        return true;
                    }
                }
                None => {}
            }
        }

        let target = self.board.get(index_clicked as usize + 1);
        match target {
            Some(tile) => {
                if tile.empty {
                    return true;
                }
            }
            None => {}
        }

        if index_clicked >= self.width {
            let target = self.board.get(index_clicked as usize - self.width as usize);
            match target {
                Some(tile) => {
                    if tile.empty {
                        return true;
                    }
                }
                None => {}
            }
        }

        let target = self.board.get(index_clicked as usize + self.width as usize);
        match target {
            Some(tile) => {
                if tile.empty {
                    return true;
                }
            }
            None => {}
        }

        false
    }

    fn make_move(&mut self, index_clicked: u8) {
        let empty = Game::empty_index(self);

        self.board.swap(empty as usize, index_clicked as usize);
        self.move_count += 1;
    }

    fn empty_index(&self) -> u8 {
        for (i, tile) in self.board.iter().enumerate() {
            if tile.empty {
                return i as u8;
            }
        }
        return 0;
    }
}