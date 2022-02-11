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

                <div class="row">
                    <div id="stats" class="col-sm-3">
                        <table>
                            <tr>
                                <td>{"Move count"}</td>
                                <td>{self.game.move_count}</td>
                            </tr>
                            <tr>
                                <td>{"High score"}</td>
                                <td>{self.game.high_score}</td>
                            </tr>
                        </table>
                    </div>
                    <div id="restart" class="col-sm-2">
                        <button class="btn btn-primary" onclick={link.callback(|_| Msg::Restart)} >{ if self.game.high_score == 0 {"Start"} else {"Restart"} }</button>
                    </div>
                </div>
                <div class="row">
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
                    text-decoration: none;
                }}

                #tile {{
                    width: 100%;
                    height: 100%;
                    outline: 1px solid #5488BA;
                }}

                #restart {{
                    visibility: {visibility};
                }}

                .empty {{
                    background-color: black;
                }}

                .full {{
                    background-color: #54BAB9;
                    color: #FBF8F1;
                    text-align: center;
                    vertical-align: middle;
                    line-height: {child_height}px;
                    font-size: {font_size}px;
                    text-decoration: none;
                }}

                body {{
                    background-color: #F7ECDE;
                }}

                .btn-primary {{
                    background-color: #3F9C9B;
                    outline: none;
                }}

                .btn-primary:hover {{
                    background-color: #3F9C6D;
                    outline: none;
                }}

                #tile:hover {{
                    outline: 2px solid #BA5455;
                    z-index: 9999;
                    position: relative;
                }}
            ", 
            parent_width = self.game.width,
            parent_height = self.game.height as u32 * self.tile_size,
            child_width = self.tile_size,
            child_height = self.tile_size,
            font_size = self.tile_size as f32 * 0.75,
            visibility = visible,
            )}
            </style>
            </>
        }
    }
}
