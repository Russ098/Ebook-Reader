use druid::{Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily, commands, AppDelegate, DelegateCtx, Target, Command, Handled, ImageBuf, Widget, WidgetExt, Event, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, WidgetId, WindowHandle};
use druid::text::{RichText, Attribute};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::from_utf8;
use druid::Event::WindowSize;
use druid::im::Vector;
use druid::widget::{Image, SizedBox};
use epub::archive::EpubArchive;
use imagesize::{size, ImageSize, blob_size};
use druid::piet::ImageFormat;
use image::imageops::resize;
use native_dialog::{MessageDialog, MessageType};
use voca_rs::Voca;
use crate::view::build_widget;

const SIZE_FONT: f64 = 40.0;

//TODO: implemenatare una struttura che gestisca i capitolo secondo formattazione html v[0]="<p>Test<p>" v[1]="<img>....<img>"
#[derive(Clone, Data, Lens)]
pub struct ImageOfChapter {
    pub image: Vector<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageOfChapter {
    pub fn new() -> Self {
        Self {
            image: Vector::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn from(image: Vector<u8>, width: usize, height: usize) -> Self {
        Self {
            image,
            width,
            height,
        }
    }
}

pub struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
    window_size: f64,
}

impl Rebuilder {
    pub fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
            window_size: 1100.,
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
            self.inner = build_widget(data);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if(data.window_size != self.window_size ){
            data.window_size = self.window_size;
        }
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.window_size = ctx.window().get_size().width;
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

#[derive(Clone, Data, Lens)]
pub struct Chapter {
    pub text: String,
    pub images: Vector<ImageOfChapter>,
}

impl Chapter {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            images: Vector::<ImageOfChapter>::new(),
        }
    }

    pub fn load_params(txt: String, imgs: Vector<ImageOfChapter>) -> Self {
        Self {
            text: txt,
            images: imgs,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub font_size: String,
    pub ebook: Vector<Chapter>,
    pub current_chapter_index: usize,
    pub window_size: f64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: Vector::<Chapter>::new(),
            current_chapter_index: 0,
            window_size: 1100.,
        }
    }

    pub fn setWindowSize(&mut self,size : f64){
        self.window_size = size;
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
    pub fn click_edit_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
    }

    pub fn click_save_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
    }

    pub fn click_single_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
    }

    pub fn click_double_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
    }

    pub fn click_previous_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
    }

    pub fn click_next_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            let dialog = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert();

        } else {
            //TODO: Fare la vera funzione

            println!("Ebook non vuoto");
        }
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
        /*let mut x = 0;
        let mut v;*/

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            //println!("{}", file_info.path().display());
            match EpubArchive::new(file_info.clone().path())
            {
                Ok(mut archive) => {
                    data.ebook.clear();
                    data.current_chapter_index = 0;
                    let mut j = 0;
                    for f in archive.files.clone() {
                        if f.contains("OEBPS") && f.contains("htm.html") {
                            data.ebook.push_back(Chapter::new());
                            //TODO: revisionare una volta fatto il salvataggio (per la questione relativa al segnalibro:
                            // quando chiudo l'app, la prossima riapertura mi riporta all'ultima pagina letta (?))
                            // let res = archive.get_entry_as_str(f);
                            let res = archive.get_entry_as_str(f);
                            if res.is_ok() {
                                let img_occ = res.as_ref().unwrap().matches("<img").count();
                                let mut pos = res.as_ref().unwrap().find("<img");
                                if img_occ > 0 {
                                    if pos.is_some() {
                                        let mut displacement: usize = 0;
                                        for i in 0..img_occ {
                                            let mut s1 = String::from("OEBPS/");
                                            let mut app = res.as_ref().unwrap()[pos.unwrap() + 3 + displacement..].find("src=");
                                            for c in res.as_ref().unwrap()[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].chars() {
                                                if c == '"' {
                                                    break;
                                                } else {
                                                    s1.push(c);
                                                }
                                            }
                                            let (width, height) = match blob_size(archive.get_entry(s1.clone()).unwrap().as_slice()) {
                                                Ok(dim) => {(dim.width, dim.height)},
                                                Err(why) => {
                                                    println!("Error getting dimensions: {:?}", why);
                                                    (0, 0)
                                                }
                                            };
                                            /*v = archive.get_entry(s1.clone());
                                            if x == 0{
                                                if let Ok(mut file) = File::create("img.jpeg") {
                                                    file.write_all(&v.unwrap().clone().as_slice());
                                                }
                                            }*/
                                            let mut  r;
                                            if s1.to_lowercase().contains("jpg") || s1.to_lowercase().contains("jpeg"){
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Jpeg).unwrap();
                                            } else if s1.to_lowercase().contains("png") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Png).unwrap();
                                            } else if s1.to_lowercase().contains("gif") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Gif).unwrap();
                                            }else if s1.to_lowercase().contains("webp") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::WebP).unwrap();
                                            }else if s1.to_lowercase().contains("pnm") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Pnm).unwrap();
                                            }else if s1.to_lowercase().contains("tiff") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tiff).unwrap();
                                            }else if s1.to_lowercase().contains("tga") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tga).unwrap();
                                            }else if s1.to_lowercase().contains("bmp") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Bmp).unwrap();
                                            }else if s1.to_lowercase().contains("ico") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Ico).unwrap();
                                            }else if s1.to_lowercase().contains("hdr") {
                                                r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Hdr).unwrap();
                                            }else {
                                                panic!("Formato non supportato");
                                            }
                                            let result = r.into_bytes();
                                            data.ebook[j].images.push_back(ImageOfChapter::from(Vector::from(result), width, height));
                                            let resapp = res.as_ref().unwrap()[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                            displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                            pos = resapp;
                                        }
                                    }
                                }
                                data.ebook[j].text.push_str(res.unwrap().clone().as_str());
                                //let c = Chapter::new();
                                //data.ebook.push_back();
                                /*data.ebook = translated_html.clone();
                                data.rich_text = RichText::new(ArcStr::from(data.ebook.clone())).with_attribute(.., Attribute::FontSize(KeyOrValue::Concrete(40.)));*/
                                // println!("{}", res.unwrap());
                            }
                            j += 1;
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

