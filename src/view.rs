use druid::{widget::{Flex}, Widget, WidgetExt, Color, UnitPoint, FileDialogOptions, FileSpec, lens, Rect, ImageBuf};

use crate::data::*;
use druid::widget::{TextBox, Button, RawLabel, Scroll, SizedBox, LensWrap, Image};
use voca_rs::strip::strip_tags;
use voca_rs::Voca;

fn option_row() -> impl Widget<AppState> {

    let epub= FileSpec::new("Epub file", &["epub"]);
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
        .border(Color::GRAY,0.5)
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
        .border(Color::GRAY,0.5)
}

pub fn build_ui() -> impl Widget<AppState> {
    let mut c = Flex::column();
    c.add_child(option_row());
    c.add_flex_child(Rebuilder::new().center(),1.0);
    c.add_child(settings_row());
    return c;
}

//TODO: creare una nuova fz che implementi Widget<AppState> e che si modifichi in base al contenuto del Vector (contenuto nella "nuova"
// struct in data.rs) andando a creare Widget::Image (per le immagini) e Widget::RawLabel (o Label) per il testo
pub fn build_widget(state: &AppState) /*-> Box<dyn Widget<AppState>>*/ {
    //titolo(?), testo, immagini
    //SizedBox::new(Scroll::new(RawLabel::new().lens(AppState::rich_text))).expand_height()
    //let png_data = ImageBuf::from_data(include_bytes!("./assets/PicWithAlpha.png")).unwrap();
    //let mut flex_content = Flex::column();
    let mut src = "".to_string();
    let mut src_pos;

    if state.ebook.len() > 0 {
        for element in state.ebook[state.current_chapter_index].text.split("\n"){
            if element.contains("img"){
                src_pos = element.find("src=");
                if src_pos.is_some(){
                    for c in element[src_pos.unwrap() + 5 ..].chars() {
                        if c == '"' {
                            break;
                        } else {
                            src.push(c);
                        }
                    }
                    println!("SRC_: {}", src);
                }
            }
            println!("{}", element);
            //TODO: inserire il contenuto ricavato nei Widget appositi (TextBox o simili e Immagini). Attenzione che per ora si hanno
            // tutti i capitoli, ma all'apertura bisogna mostrare solo il contenuto del primo

            //println!("{}", strip_tags(element));
        }
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
}