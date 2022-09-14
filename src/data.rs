use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands};
use druid::text::{RichText, Attribute};

const SIZE_FONT: f64 = 40.0;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    font_size: String,
    rich_text: RichText,
    ebook: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: String::new(),
            rich_text: RichText::new(ArcStr::from(""))
                .with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(SIZE_FONT)))
                .with_attribute(.., Attribute::FontFamily(FontFamily::SANS_SERIF)),
        }
    }
    pub fn click_plus_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.plus();
    }
    fn plus(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() + 1.;
        self.font_size = new_size.to_string();
        //self.rich_text = self.rich_text.clone().with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        self.rich_text.add_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
    }
    pub fn click_min_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.min();
    }
    fn min(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() - 1.;
        self.font_size = new_size.to_string();
        //self.rich_text = self.rich_text.clone().with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        self.rich_text.add_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
    }
    pub fn click_open_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    let first_line = s.lines().next().unwrap_or("");
                    data.ebook = first_line.to_owned();
                }
                Err(e) => {

                }
            }
        }
    }
}


