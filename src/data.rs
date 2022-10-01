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
pub struct ImageOfPage {
    pub image: Vector<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageOfPage {
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
        if (data.window_size != self.window_size) {
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
pub struct Page {
    pub text: String,
    pub images: Vector<ImageOfPage>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            images: Vector::<ImageOfPage>::new(),
        }
    }

    pub fn load_params(txt: String, imgs: Vector<ImageOfPage>) -> Self {
        Self {
            text: txt,
            images: imgs,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub font_size: String,
    pub ebook: Vector<Page>,
    pub current_page: usize,
    pub window_size: f64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: Vector::<Page>::new(),
            current_page: 0,
            window_size: 1100.,
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

    //TODO: Resettare il re
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
                    data.current_page = 0;
                    let mut page_no = 0;
                    let mut page_not_ended = false;

                    for f in archive.files.clone() {
                        let mut pageno_found = false;
                        if f.contains("OEBPS") && f.contains("htm.html") {
                            data.ebook.push_back(Page::new());

                            println!("File aperto {} pageno: {}", f, page_no);
                            let res = archive.get_entry_as_str(f);

                            if res.is_ok() {
                                let init = res.as_ref().unwrap().find("<body");
                                let page_occ = res.as_ref().unwrap()[init.unwrap()..].matches("<span class=\"x-ebookmaker-pageno\"").count();

                                if page_occ > 0 {
                                    //controllo per vedere se il file html aperto è suddiviso in più pagine
                                    pageno_found = true;
                                    let mut pos_pageno = res.as_ref().unwrap()[init.unwrap()..].find("<span class=\"x-ebookmaker-pageno\"");

                                    for i in 0..page_occ {

                                        if page_not_ended {

                                            let text = res.as_ref().unwrap()[init.unwrap()..]._substr(0 , pos_pageno.unwrap());
                                            let img_occ = text.matches("<img").count();
                                            let mut pos = text.find("<img");
                                            if img_occ > 0 {
                                                if pos.is_some() {
                                                    let mut displacement: usize = 0;
                                                    for i in 0..img_occ {
                                                        let mut s1 = String::from("OEBPS/");
                                                        let mut app = text[pos.unwrap() + 3 + displacement..].find("src=");
                                                        for c in text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].chars() {
                                                            if c == '"' {
                                                                break;
                                                            } else {
                                                                s1.push(c);
                                                            }
                                                        }
                                                        let (width, height) = match blob_size(archive.get_entry(s1.clone()).unwrap().as_slice()) {
                                                            Ok(dim) => { (dim.width, dim.height) }
                                                            Err(why) => {
                                                                println!("Error getting dimensions: {:?}", why);
                                                                (0, 0)
                                                            }
                                                        };

                                                        let mut r;

                                                        if s1.to_lowercase().contains("jpg") || s1.to_lowercase().contains("jpeg") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Jpeg).unwrap();
                                                        } else if s1.to_lowercase().contains("png") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Png).unwrap();
                                                        } else if s1.to_lowercase().contains("gif") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Gif).unwrap();
                                                        } else if s1.to_lowercase().contains("webp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::WebP).unwrap();
                                                        } else if s1.to_lowercase().contains("pnm") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Pnm).unwrap();
                                                        } else if s1.to_lowercase().contains("tiff") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tiff).unwrap();
                                                        } else if s1.to_lowercase().contains("tga") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tga).unwrap();
                                                        } else if s1.to_lowercase().contains("bmp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Bmp).unwrap();
                                                        } else if s1.to_lowercase().contains("ico") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Ico).unwrap();
                                                        } else if s1.to_lowercase().contains("hdr") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Hdr).unwrap();
                                                        } else {
                                                            panic!("Formato non supportato");
                                                        }
                                                        let result = r.into_bytes();
                                                        data.ebook[page_no-1].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                        let resapp = text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                        displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                        pos = resapp;
                                                    }
                                                }
                                            }

                                            data.ebook[page_no-1].text.push_str(text.as_str());
                                            page_not_ended = false;
                                        }

                                        if i == page_occ - 1 {

                                            page_not_ended = true;
                                            let text = res.as_ref().unwrap()[init.unwrap()..]._substr(pos_pageno.unwrap(), res.as_ref().unwrap().len());
                                            let img_occ = text.matches("<img").count();
                                            let mut pos = text.find("<img");
                                            if img_occ > 0 {
                                                if pos.is_some() {
                                                    let mut displacement: usize = 0;
                                                    for i in 0..img_occ {
                                                        let mut s1 = String::from("OEBPS/");
                                                        let mut app = text[pos.unwrap() + 3 + displacement..].find("src=");
                                                        for c in text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].chars() {
                                                            if c == '"' {
                                                                break;
                                                            } else {
                                                                s1.push(c);
                                                            }
                                                        }
                                                        let (width, height) = match blob_size(archive.get_entry(s1.clone()).unwrap().as_slice()) {
                                                            Ok(dim) => { (dim.width, dim.height) }
                                                            Err(why) => {
                                                                println!("Error getting dimensions: {:?}", why);
                                                                (0, 0)
                                                            }
                                                        };

                                                        let mut r;

                                                        if s1.to_lowercase().contains("jpg") || s1.to_lowercase().contains("jpeg") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Jpeg).unwrap();
                                                        } else if s1.to_lowercase().contains("png") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Png).unwrap();
                                                        } else if s1.to_lowercase().contains("gif") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Gif).unwrap();
                                                        } else if s1.to_lowercase().contains("webp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::WebP).unwrap();
                                                        } else if s1.to_lowercase().contains("pnm") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Pnm).unwrap();
                                                        } else if s1.to_lowercase().contains("tiff") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tiff).unwrap();
                                                        } else if s1.to_lowercase().contains("tga") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tga).unwrap();
                                                        } else if s1.to_lowercase().contains("bmp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Bmp).unwrap();
                                                        } else if s1.to_lowercase().contains("ico") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Ico).unwrap();
                                                        } else if s1.to_lowercase().contains("hdr") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Hdr).unwrap();
                                                        } else {
                                                            panic!("Formato non supportato");
                                                        }
                                                        let result = r.into_bytes();
                                                        data.ebook[page_no].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                        let resapp = text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                        displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                        pos = resapp;
                                                    }
                                                }
                                            }

                                            data.ebook[page_no].text.push_str(text.as_str());
                                            page_no += 1;

                                        } else {

                                            let next_page = res.as_ref().unwrap()[pos_pageno.unwrap() + 34 ..].find("<span class=\"x-ebookmaker-pageno\"");
                                            let text = res.as_ref().unwrap()[init.unwrap()..]._substr(pos_pageno.unwrap(), next_page.unwrap());
                                            let img_occ = text.matches("<img").count();
                                            let mut pos = text.find("<img");
                                            if img_occ > 0 {
                                                if pos.is_some() {
                                                    let mut displacement: usize = 0;
                                                    for i in 0..img_occ {
                                                        let mut s1 = String::from("OEBPS/");
                                                        let mut app = text[pos.unwrap() + 3 + displacement..].find("src=");
                                                        for c in text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].chars() {
                                                            if c == '"' {
                                                                break;
                                                            } else {
                                                                s1.push(c);
                                                            }
                                                        }
                                                        let (width, height) = match blob_size(archive.get_entry(s1.clone()).unwrap().as_slice()) {
                                                            Ok(dim) => { (dim.width, dim.height) }
                                                            Err(why) => {
                                                                println!("Error getting dimensions: {:?}", why);
                                                                (0, 0)
                                                            }
                                                        };

                                                        let mut r;

                                                        if s1.to_lowercase().contains("jpg") || s1.to_lowercase().contains("jpeg") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Jpeg).unwrap();
                                                        } else if s1.to_lowercase().contains("png") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Png).unwrap();
                                                        } else if s1.to_lowercase().contains("gif") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Gif).unwrap();
                                                        } else if s1.to_lowercase().contains("webp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::WebP).unwrap();
                                                        } else if s1.to_lowercase().contains("pnm") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Pnm).unwrap();
                                                        } else if s1.to_lowercase().contains("tiff") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tiff).unwrap();
                                                        } else if s1.to_lowercase().contains("tga") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tga).unwrap();
                                                        } else if s1.to_lowercase().contains("bmp") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Bmp).unwrap();
                                                        } else if s1.to_lowercase().contains("ico") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Ico).unwrap();
                                                        } else if s1.to_lowercase().contains("hdr") {
                                                            r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Hdr).unwrap();
                                                        } else {
                                                            panic!("Formato non supportato");
                                                        }
                                                        let result = r.into_bytes();
                                                        data.ebook[page_no].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                        let resapp = text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                        displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                        pos = resapp;
                                                    }
                                                }
                                            }

                                            data.ebook[page_no].text.push_str(text.as_str());
                                            pos_pageno = next_page;

                                            data.ebook.push_back(Page::new());
                                            page_no += 1;
                                        }


                                    }
                                }

                                //TODO: revisionare una volta fatto il salvataggio (per la questione relativa al segnalibro:
                                // quando chiudo l'app, la prossima riapertura mi riporta all'ultima pagina letta (?))
                                // let res = archive.get_entry_as_str(f);
                                else {
                                    let img_occ = res.as_ref().unwrap()[init.unwrap()..].matches("<img").count();
                                    let mut pos = res.as_ref().unwrap()[init.unwrap()..].find("<img");
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
                                                    Ok(dim) => { (dim.width, dim.height) }
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
                                                let mut r;
                                                if s1.to_lowercase().contains("jpg") || s1.to_lowercase().contains("jpeg") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Jpeg).unwrap();
                                                } else if s1.to_lowercase().contains("png") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Png).unwrap();
                                                } else if s1.to_lowercase().contains("gif") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Gif).unwrap();
                                                } else if s1.to_lowercase().contains("webp") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::WebP).unwrap();
                                                } else if s1.to_lowercase().contains("pnm") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Pnm).unwrap();
                                                } else if s1.to_lowercase().contains("tiff") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tiff).unwrap();
                                                } else if s1.to_lowercase().contains("tga") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Tga).unwrap();
                                                } else if s1.to_lowercase().contains("bmp") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Bmp).unwrap();
                                                } else if s1.to_lowercase().contains("ico") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Ico).unwrap();
                                                } else if s1.to_lowercase().contains("hdr") {
                                                    r = image::load_from_memory_with_format(archive.get_entry(s1.clone()).unwrap().as_slice(), image::ImageFormat::Hdr).unwrap();
                                                } else {
                                                    panic!("Formato non supportato");
                                                }
                                                let result = r.into_bytes();
                                                data.ebook[page_no].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                let resapp = res.as_ref().unwrap()[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                pos = resapp;
                                            }
                                        }
                                    }
                                    data.ebook[page_no].text.push_str(res.unwrap().clone().as_str());

                                    page_no += 1;
                                }
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

