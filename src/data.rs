use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands, AppDelegate, DelegateCtx, Target, Command, Handled};
use druid::text::{RichText, Attribute};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
            println!("{}",file_info.path().display());
            match EpubDoc::new(file_info.path()) {
                Ok(mut s) => {
                    if let Some(title) = s.mdata("title") {
                        println!("Book title: {}", title);
                    } else {
                        println!("Book title not found");
                    }
                    println!("Num Pages: {}\n", s.get_num_pages());

                    {
                        println!("resources:\n");
                        for (k, v) in s.resources.iter() {
                            println!("{}: {}\n * {}\n", k, v.1, v.0.to_str().unwrap());
                            let fileref = file_info.path().to_str().unwrap().to_owned().replace(".epub", "\\") + v.0.as_path().to_str().unwrap();
                            println!("percorso: {}",fileref);
                            let file = File::open(Path::new(fileref.as_str()));
                            match file {
                                Ok(mut f) => {
                                    f.read_to_string(&mut data.ebook.clone());
                                }
                                Err(e) => {
                                    println!("Errore apertura file xml: {}", e);
                                }
                            }
                        }
                        println!("fine resources");
                    }
/*
                    while let Ok(_) = s.go_next() {
                        println!("ID: {}", s.get_current_id().unwrap());
                        let current = s.get_current_str();
                        match current {
                            Ok(v) => {
                                println!("Value {:?}\n", v);
                                data.ebook.push_str(v.as_str());
                            },
                            Err(e) => println!("Text Err {:?}\n", e),
                        }
                    }
 */
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



