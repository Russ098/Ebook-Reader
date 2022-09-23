use std::array::TryFromSliceError;
use std::sync::Arc;
use druid::{widget::{Flex}, Widget, WidgetExt, Color, UnitPoint, FileDialogOptions, FileSpec, lens, Rect, ImageBuf, Vec2};
use druid::im::Vector;
use druid::piet::ImageFormat;

use crate::data::*;
use druid::widget::{TextBox, Button, RawLabel, Scroll, SizedBox, LensWrap, Image, FillStrat, Label};
use imagesize::size;
use voca_rs::strip::strip_tags;
use voca_rs::Voca;

fn option_row() -> impl Widget<AppState> {
    let epub = FileSpec::new("Epub file", &["epub"]);
    let default_save_name = String::from("MyFile.epub");

    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![epub])
        .default_type(epub)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target for this lovely file")
        .button_text("Export");

    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.epub")
        .name_label("Source")
        .title("Where did you put that file?")
        .button_text("Import");

    let open_button = Button::new("Open").padding(5.0).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });
    let edit_button = Button::new("Edit").padding(5.0).on_click(AppState::click_edit_button);
    let save_button = Button::new("Save").padding(5.0).on_click(AppState::click_save_button);
    Flex::row()
        .with_child(open_button)
        .with_child(edit_button)
        .with_child(save_button)
        .align_left()
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

fn settings_row() -> impl Widget<AppState> {
    let single_page_button = Button::new("Single Page").padding(5.0).on_click(AppState::click_single_page_button);
    let double_page_button = Button::new("Double Page").padding(5.0).on_click(AppState::click_double_page_button);
    let plus_button = Button::new("+").padding(5.0).on_click(AppState::click_plus_button);
    let min_button = Button::new("-").padding(5.0).on_click(AppState::click_min_button);
    let edit_size_text = TextBox::new()
        .with_placeholder("50")
        .lens(AppState::font_size);
    Flex::row()
        .with_child(single_page_button)
        .with_child(double_page_button)
        .with_child(min_button)
        .with_child(edit_size_text)
        .with_child(plus_button)
        .align_right()
        .align_vertical(UnitPoint::BOTTOM)
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

pub fn build_ui() -> impl Widget<AppState> {
    let mut c = Flex::column();
    c.add_child(option_row());
    c.add_flex_child(Rebuilder::new().center(), 1.0);
    c.add_child(settings_row());
    return c;
}

//TODO: creare una nuova fz che implementi Widget<AppState> e che si modifichi in base al contenuto del Vector (contenuto nella "nuova"
// struct in data.rs) andando a creare Widget::Image (per le immagini) e Widget::RawLabel (o Label) per il testo
pub fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    //titolo(?), testo, immagini
    //SizedBox::new(Scroll::new(RawLabel::new().lens(AppState::rich_text))).expand_height()
    let mut c = Flex::column();
    let mut src = "".to_string();
    let mut i = 0 as usize;
    // let v2 = state.ebook[state.current_chapter_index].images[i].clone().image;
    let mut v: Vec<u8> = vec![];
    let mut pixels_vec = Vec::new();
    let mut image_buf;

    if state.ebook.len() > 0 {
        for element in state.ebook[state.current_chapter_index].text.split("\n") {
            if element.contains("img") {
                println!("Element: {} - i: {}", element, i);
                //println!("{}", state.ebook[state.current_chapter_index].images.len());
                for pixel in state.ebook[state.current_chapter_index].images[i].image.clone(){
                    pixels_vec.push(pixel);
                }
                //println!("len: {} - len2: {}", pixels_vec.len(), state.ebook[state.current_chapter_index].images[i].image.clone().len());
                // println!("Pixel len: {} -- Format factor: {}", pixels_vec.len(), ImageFormat::Rgb.bytes_per_pixel());
                match pixels_vec.len()/(state.ebook[state.current_chapter_index].images[i].width * state.ebook[state.current_chapter_index].images[i].height) {
                    1 => {image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Grayscale, state.ebook[state.current_chapter_index]
                        .images[i].width, state.ebook[state.current_chapter_index].images[i].height);},
                    3 => {image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Rgb, state.ebook[state.current_chapter_index]
                        .images[i].width, state.ebook[state.current_chapter_index].images[i].height);},
                    4 =>{image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::RgbaPremul, state.ebook[state.current_chapter_index]
                            .images[i].width, state.ebook[state.current_chapter_index].images[i].height);}
                    _ => {panic!("Unable to process the image")}
                }

                let mut img = Image::new(image_buf).fill_mode(FillStrat::Cover);
                let mut sized = SizedBox::new(img);
                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();
                c.add_child(container);
                i += 1;
                pixels_vec.clear();
            } else {
                let _string = strip_tags(element);
                let rl = Label::new(_string.clone());
                println!("{}", _string.clone());
                c.add_child(rl);
            }
        }
    }

    let mut scroll = Scroll::new(c);
    println!("Scroll test: {}", scroll.scroll_by(Vec2::new(0f64, 0f64))); //TODO: risolvere?
    SizedBox::new(scroll).expand_height().boxed() //TODO: verificare/risolvere(?) se con il mouse sfarfalla scrollando orizzontalmente
}

/*let mut img = Image::new(png_data).fill_mode(state.fill_strat);
if state.interpolate {
img.set_interpolation_mode(state.interpolation_mode)
}
if state.clip {
img.set_clip_area(Some(Rect::new(
state.clip_x,
state.clip_y,
state.clip_x + state.clip_width,
state.clip_y + state.clip_height,
)));
}
let mut sized = SizedBox::new(img);
if state.fix_width {
sized = sized.fix_width(state.width)
}
if state.fix_height {
sized = sized.fix_height(state.height)
}
sized.border(Color::grey(0.6), 2.0).center().boxed()*/
