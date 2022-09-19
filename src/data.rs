use std::error::Error;
use std::fmt::Debug;
use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands, AppDelegate, DelegateCtx, Target, Command, Handled, ImageBuf};
use druid::text::{RichText, Attribute};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use druid::im::Vector;
use druid::widget::Image;
use epub::archive::EpubArchive;
use html2text::{from_read, from_read_rich};

const SIZE_FONT: f64 = 40.0;

//TODO: implemenatare una struttura che gestisca i capitolo secondo formattazione html v[0]="<p>Test<p>" v[1]="<img>....<img>"
#[derive(Clone, Data, Lens)]
pub struct Chapter{
    text : String,
    images : Vector<Vector<u8>>
}

impl Chapter{
    pub fn new() -> Self{
        Self{
            text : String::new(),
            images : Vector::<Vector<u8>>::new()
        }
    }

    pub fn load_params(txt : String, imgs : Vector<Vector<u8>>) -> Self{
        Self{
            text : txt,
            images : imgs
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub font_size: String,
    rich_text: RichText,
    ebook: Vector<Chapter>,
    current_chapter_index : usize
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: Vector::<Chapter>::new(),
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
        /*if self.ebook.is_empty(){
            self.rich_text = RichText::new(ArcStr::from(new_size.to_string().as_str())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }else{
            self.rich_text = RichText::new(ArcStr::from(self.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }*/
    }
    pub fn click_min_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.min();
    }
    fn min(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() - 1.;
        self.font_size = new_size.to_string();
        /*if self.ebook.is_empty(){
            self.rich_text = RichText::new(ArcStr::from(new_size.to_string().as_str())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }else{
            self.rich_text = RichText::new(ArcStr::from(self.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(new_size)));
        }*/
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
                            let res = archive.get_entry_as_str(f);
                            if res.is_ok(){
                                //println!("{}", res.unwrap());
                                //TODO: riempire la struct che contiene Vector con il contenuto di ogni capitolo
                                let mut s = res.as_ref().unwrap().find("</head>");
                                if s.is_some(){
                                    let s = s.unwrap()+7/*"/<head>".len()*/;
                                    let img_occ = res.as_ref().unwrap().matches("<img").count();
                                    if img_occ > 0{
                                        let pos = res.as_ref().unwrap().find("<img");
                                        if pos.is_some() {
                                            for i in 0..img_occ{
                                                for c in res.as_ref().unwrap()[pos.unwrap()..].chars(){
                                                    //TODO: prelevare la stringa di riferimento per l'immagine lavorando carattere per carattere
                                                }
                                            }
                                        }
                                    }
                                    let c = Chapter::new();
                                    //data.ebook.push_back();
                                }
                                /*data.ebook = translated_html.clone();
                                data.rich_text = RichText::new(ArcStr::from(data.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(40.)));*/
                                // println!("{}", res.unwrap());
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

