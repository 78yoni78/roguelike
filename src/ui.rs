use tcod::{
    Console,
    Color,
    colors,
    BackgroundFlag,
    TextAlignment,
};

trait Render {
    fn render(&self, con: &mut dyn Console);
}

struct Bar {
    name: String,
    pos: (u16, u16),
    width: u16,
    value: u32,
    maximum: u32,
    empty_color: Color,
    fill_color: Color,
}

impl Render for Bar {
    fn render(&self, con: &mut dyn Console) {
        let fill_width = (self.value as f32 / self. maximum as f32 * self.width as f32) as i32;
        //  render the empty part
        con.set_default_background(self.empty_color);
        con.rect(self.pos.0 as i32, self.pos.1 as i32, self.width as i32, 1, false, BackgroundFlag::Overlay);
        //  render the full part
        con.set_default_background(self.fill_color);
        if fill_width > 0 { con.rect(self.pos.0 as i32, self.pos.1 as i32, fill_width, 1, false, BackgroundFlag::Overlay); }
        //  render some centered text with the values
        con.set_default_foreground(tcod::colors::WHITE);
        (con as &dyn Console).print_ex( //  for some reason con.print_ex doesnt compile
            self.pos.0 as i32 + self.width as i32 / 2, self.pos.1 as i32, 
            BackgroundFlag::Overlay, TextAlignment::Center,
            &format!("{}: {}/{}", &self.name, self.value, self.maximum),
        );

    }
}



struct UI {
    bars: Vec<Bar>,
}

impl UI {
    pub fn draw(&self, con: &mut dyn Console) {
        for bar in &self.bars {
            bar.render(con);
        }
    }
}

