use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;

mod tile;
use tile::Tile;

pub enum Msg {
    ClickTile(u8, u8),
    Restart,
}

pub struct App {
    board: Vec<Tile>,
    width: u8,
    height: u8,
    solved: bool,
    tile_size: u8
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let width = 4;
        let height = 4;

        let mut new = App {
            board: Vec::new(),
            width: width,
            height: height,
            solved: false,
            tile_size: 50
        };

        let order = App::random_legal_order(width, height);

        let max_value = order.len();
        for i in order {
            new.board.push(Tile::new(i, i == max_value as u8));
        }

        new
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickTile(index, value) => {
                log::info!("Clicked {:?}", self.board[index as usize]);

                if self.is_legal_move(index) && !self.solved {
                    self.make_move(index);

                    let order: Vec<u8> = self.board.iter().map(|tile| tile.id).clone().collect();
                    self.solved = App::is_solved(&order);
                }

                log::info!("Is legal move: {}", self.is_legal_move(index));

                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Restart => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class = "board">
            {self.style()}
            <h1>{"15 puzzle"}</h1>
                <div class = "parent flexbox">
                    {
                        for self.board.iter().enumerate().map(|(i, tile)|
                        html! {
                            <a class = "child" onclick={
                                let index = i.clone();
                                let value = tile.id.clone();
                                link.callback(move |_| Msg::ClickTile(index as u8, value as u8))
                            }>
                            {tile.view()}
                            </a>
                        }
                        )
                    }
                </div>
            </div>
        }
    }
}

impl App {

    fn style(&self) -> Html {
        html! {
            <>
            <style>
            {format!("
                .flexbox {{
                    display: flex;
                    width: {parent_width}px;
                }}

                .parent {{
                    height: {parent_height}px;
                }}

                .child {{
                    width: {child_width}px;
                    height: {child_height}px;
                    display: inline-block;
                    flex: 0 0 50px;
                }}

            ", 
            parent_width = self.width * self.tile_size,
            parent_height = self.width * self.tile_size,
            child_width = self.tile_size,
            child_height = self.tile_size,
            )}
            </style>
            </>
        }
    }

    fn random_legal_order(width: u8, height: u8) -> Vec<u8> {
        let mut order: Vec<u8> = Vec::new();

        for i in 1..(width * height + 1) {
            order.push(i);
        }

        let mut rng = thread_rng();
        order.shuffle(&mut rng);

        while !App::is_legal_order(&order, width) && !App::is_solved(&order) {
            order.shuffle(&mut rng);
        }

        order
    }

    fn is_legal_order(order: &Vec<u8>, width: u8) -> bool {
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

        let blank_layer = (blank_index / 4) + 1;

        return (inversions % 2 == 0) != (blank_layer % 2 == 0);
    }

    fn is_solved(order: &Vec<u8>) -> bool {
        for (i, value) in order.iter().enumerate() {
            if &(i as u8) != value {
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
        let empty = App::empty_index(self);

        self.board.swap(empty as usize, index_clicked as usize);
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
