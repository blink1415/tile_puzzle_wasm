use yew::prelude::*;

#[derive(Debug)]
pub struct Tile {
    pub filename: String,
    pub empty: bool,
    pub id: u8,
}

impl Tile {
    pub fn view(&self) -> Html {
        html! {
            <>
                <img src = { self.filename.clone() } id = "tile" />
                //<p>{self.empty}</p>
                //<p>{self.id}</p>
            </>
        }
    }

    pub fn new(id: u8, empty: bool) -> Tile {
        Tile {
            filename: format!("assets/{}.png", id),
            empty: empty,
            id: id,
        }
    }
}
