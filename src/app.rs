use yew::prelude::*;

mod game;
use game::*;

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
        App {
            game: Game::new(4, 4),

            tile_size: 100,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickTile(index) => {
                self.game.click_tile(index)
            },
            Msg::Restart => {
                self.game.restart()
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class = "board container">
            {self.style()}
            <h1>{"15 puzzle"}</h1>
            <div id="stats">
            <p><strong>{"Move count: "}</strong>{self.game.move_count} </p>
            </div>
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
                <div id="restart">
                    <button onclick={link.callback(|_| Msg::Restart)} >{ if self.game.high_score == 0 {"Start"} else {"Restart"} }</button>
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
