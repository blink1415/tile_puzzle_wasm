use yew::prelude::*;

#[derive(Debug)]
pub struct Tile {
    pub empty: bool,
    pub id: u8,
}

impl Tile {
    pub fn view(&self) -> Html {
        let (value, class): (String, &'static str) = match self.empty {
            true => (String::new(), "rounded empty"),
            false => (self.id.to_string(), "rounded full"),
        };

        log::info!("{:?}", self);

        html! {
            <>
                <div id = "tile" class = {class}>{value}</div>
            </>
        }
    }

    pub fn new(id: u8, empty: bool) -> Tile {
        Tile {
            empty,
            id,
        }
    }

}
