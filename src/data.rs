use std::error::Error;
use std::fmt::Debug;
use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands, AppDelegate, DelegateCtx, Target, Command, Handled};
use druid::text::{RichText, Attribute};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use epub::archive::EpubArchive;
use html2text::from_read;

const SIZE_FONT: f64 = 40.0;

pub struct Ebook{
    chapters_list : Vec<String>
}

impl Ebook{
    pub fn new() -> Self{
        Self{chapters_list : Vec::new()}
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    font_size: String,
    rich_text: RichText,
    ebook: String,
    current_chapter_index : usize
    //todo current_page_index (?)
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: String::new(),
            rich_text: RichText::new(ArcStr::from("prova")).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(40.))),
            current_chapter_index : 0
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
            match EpubArchive::new(file_info.path())
            {
                Ok(mut archive) => {
                    for f in archive.files.clone(){
                        if f.contains("OEBPS") && f.contains("htm.html"){
                            println!("{}", f);
                            // let res = archive.get_entry_as_str(f);
                            let res = archive.get_entry(f);
                            if res.is_ok(){
                                let translated_html = from_read(res.unwrap().as_slice(), 50);

                                data.ebook = translated_html.clone();
                                data.rich_text = RichText::new(ArcStr::from(data.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(40.)));
                                // println!("{}", res.unwrap());
                                println!("{}", translated_html);
                            }
                        }
                    }
                }
                Err(error) => {
                    //TODO
                    println!("Error while opening archive: {}", error);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}

