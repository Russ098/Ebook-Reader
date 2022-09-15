use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands, AppDelegate, DelegateCtx, Target, Command, Handled};
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
            rich_text: RichText::new(ArcStr::from("prova")).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(40.)))
        }
    }
    pub fn click_plus_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.plus();
    }
    fn plus(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() + 1.;
        self.font_size = new_size.to_string();
        if self.ebook.is_empty(){
            self.rich_text = RichText::new(ArcStr::from(new_size.to_string().as_str())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }else{
            self.rich_text = RichText::new(ArcStr::from(self.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }
    }
    pub fn click_min_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.min();
    }
    fn min(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() - 1.;
        self.font_size = new_size.to_string();
        if self.ebook.is_empty(){
            self.rich_text = RichText::new(ArcStr::from(new_size.to_string().as_str())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }else{
            self.rich_text = RichText::new(ArcStr::from(self.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }
    }
    pub fn click_edit_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env){

    }

    pub fn click_save_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env){

    }

    pub fn click_single_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env){

    }

    pub fn click_double_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env){

    }
}
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        //if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            //if let Err(e) = std::fs::write(file_info.path(), &data[..]) {
                //println!("Error writing file: {}", e);
            //}
            //return Handled::Yes;
        //}
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    for line in s.lines(){
                        data.ebook.push_str(line);
                        data.ebook.push('\n');
                    }
                    data.rich_text = RichText::new(ArcStr::from(data.ebook.clone()))
                        .with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(data.font_size.parse::<f64>().unwrap())))
                        .with_attribute(.., Attribute::FontFamily(FontFamily::SANS_SERIF));
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}



