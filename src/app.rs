use yew::prelude::*;

mod game;
use game::*;
use game::tile::Tile;

pub enum Msg {
    ClickTile(u8),
    Restart,
}

pub struct App {
    game: Game,
    tile_size: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let width = 4;
        let height = 4;

        let order = Game::random_legal_order(width, height, false);
        let mut board: Vec<Tile> = Vec::new();

        let max_value = order.len();
        for i in order {
            board.push(Tile::new(i, i == max_value as u8));
        }

        App {
            game: Game {
                board: board,
                width: width,
                height: height,
                solved: false,
                move_count: 0
            },

            tile_size: 100,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickTile(index) => {
                self.game.click_tile(index)
            }

            Msg::Restart => {
                self.game.restart()
            },
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
                        for self.game.board.iter().enumerate().map(|(i, tile)|
                        html! {
                            <a class = "child" onclick={
                                let index = i.clone();
                                link.callback(move |_| Msg::ClickTile(index as u8))
                            }>
                            {tile.view()}
                            </a>
                        }
                        )
                    }
                </div>
                <div id="stats">
                <p> {self.game.move_count} </p>
                </div>
                <div id="restart">
                    <button onclick={link.callback(|_| Msg::Restart)} >{ "Restart" }</button>
                </div>
            </div>
        }
    }
}

impl App {

    fn style(&self) -> Html {
        let visible = match self.game.solved {
            true => "visible",
            false => "hidden",
        };

        html! {
            <>
            <style>
            {
                format!("
                .flexbox {{
                    display: grid;
                    grid-template-columns: repeat({parent_width}, {child_width}px);
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

                #tile {{
                    width: 100%;
                    height: 100%;
                }}

                #restart {{
                    visibility: {visibility};
                }}

            ", 
            parent_width = self.game.width,
            parent_height = self.game.height as u32 * self.tile_size,
            child_width = self.tile_size,
            child_height = self.tile_size,
            visibility = visible,
            )}
            </style>
            </>
        }
    }
}
