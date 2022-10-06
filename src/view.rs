use std::array::TryFromSliceError;
use std::sync::Arc;
use druid::{widget::{Flex}, Widget, WidgetExt, Color, UnitPoint, FileDialogOptions, FileSpec, lens, Rect, ImageBuf, Vec2, KeyOrValue, Size, TextAlignment};
use druid::im::Vector;
use druid::piet::ImageFormat;
use druid::text::RichText;

use crate::data::*;
use druid::widget::{TextBox, Button, RawLabel, Scroll, SizedBox, LensWrap, Image, FillStrat, Label, CrossAxisAlignment, LineBreaking, Padding};
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
    let previous_button = Button::new("Previous Page").padding(5.0).on_click(AppState::click_previous_button);
    let next_button = Button::new("Next Page").padding(5.0).on_click(AppState::click_next_button);

    let r1 = Flex::row()
        .with_child(open_button)
        .with_child(edit_button)
        .with_child(save_button)
        .align_left();

    let r2 = Flex::row()
        .with_child(previous_button)
        .with_child(next_button)
        .align_right();


    Flex::row()
        .with_flex_child(r1, 1.0)
        .with_flex_child(r2, 1.0)
        .expand_width()
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
    c.add_flex_child(Rebuilder::new(), 1.0);
    c.add_child(settings_row());
    return c;
}

//TODO: Aggiustare il controllo per la dimensione del font quando viene messa a 0 oppure cancellata dall'utente

pub fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    //titolo(?), testo, immagini
    //SizedBox::new(Scroll::new(RawLabel::new().lens(AppState::rich_text))).expand_height()
    let mut c = Flex::column();
    let mut src = "".to_string();
    let mut i = 0 as usize;
    let mut v: Vec<u8> = vec![];
    let mut pixels_vec = Vec::new();
    let mut image_buf;
    let mut scroll;
    let mut c2 = Flex::column();

    //TODO: Per i file .html che non presentano all'interno un "pageno", la pagina cambia ogni volta che cambia il file html che viene aperto.
    // Altrimenti la pagina viene riempita fin quando non si incontra un tag span che contiene "pageno".
    // NOTA: se il file .html corrente possiede del testo che non è seguito da un pageno, allora si controllerà il successivo file .html e, se contiene un pageno, si continuerà a
    // riempire, altrimenti si scriverà su una nuova pagina il file nuovo .html. DA SALVARE: Numero di pagina, indice di lettura (indice in cui ci troviamo all'interno del file),
    // numero del capitolo, eventuali flag da usare


    if state.ebook.len() > 0 {


        let mut str_page = String::new();
        str_page.push_str(state.current_page.to_string().as_str());
        str_page.push_str("\n\n");
        let rl_page = Label::new(str_page)
            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
            .with_text_alignment(TextAlignment::Center)
            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

        c.add_child(rl_page);

        for element in state.ebook[state.current_page].text.split("\n") {
            if element.contains("img") {
                for pixel in state.ebook[state.current_page].images[i].image.clone() {
                    pixels_vec.push(pixel);
                }
                match pixels_vec.len() / (state.ebook[state.current_page].images[i].width * state.ebook[state.current_page].images[i].height) {
                    1 => {
                        image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Grayscale, state.ebook[state.current_page]
                            .images[i].width, state.ebook[state.current_page].images[i].height);
                    }
                    3 => {
                        image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Rgb, state.ebook[state.current_page]
                            .images[i].width, state.ebook[state.current_page].images[i].height);
                    }
                    4 => {
                        image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::RgbaPremul, state.ebook[state.current_page]
                            .images[i].width, state.ebook[state.current_page].images[i].height);
                    }
                    _ => { panic!("Unable to process the image") }
                }


                let mut img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);


                let mut sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                            image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                c.add_child(container);
                i += 1;
                pixels_vec.clear();
            } else {
                let mut _string;

                let mut appStr = element.to_string();

                if appStr.len() >= 1 {
                    if appStr.chars().last().unwrap() == '<' {
                        appStr.replace_range(appStr.len() - 1.., "");
                    }
                }

                _string = strip_tags(appStr.as_str());


                let rl = Label::new(_string.clone())
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                c.add_child(rl);
            }
        }

        if state.double_page {
            if state.current_page + 1 < state.ebook.len() {


                let mut str_page = String::new();
                str_page.push_str((state.current_page + 1).to_string().as_str());
                str_page.push_str("\n\n");
                let rl_page = Label::new(str_page)
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_text_alignment(TextAlignment::Center)
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                c2.add_child(rl_page);


                for element in state.ebook[state.current_page + 1].text.split("\n") {
                    if element.contains("img") {
                        for pixel in state.ebook[state.current_page + 1].images[i].image.clone() {
                            pixels_vec.push(pixel);
                        }
                        match pixels_vec.len() / (state.ebook[state.current_page + 1].images[i].width * state.ebook[state.current_page + 1].images[i].height) {
                            1 => {
                                image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Grayscale, state.ebook[state.current_page + 1]
                                    .images[i].width, state.ebook[state.current_page + 1].images[i].height);
                            }
                            3 => {
                                image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::Rgb, state.ebook[state.current_page + 1]
                                    .images[i].width, state.ebook[state.current_page + 1].images[i].height);
                            }
                            4 => {
                                image_buf = ImageBuf::from_raw(pixels_vec.clone(), ImageFormat::RgbaPremul, state.ebook[state.current_page + 1]
                                    .images[i].width, state.ebook[state.current_page + 1].images[i].height);
                            }
                            _ => { panic!("Unable to process the image") }
                        }


                        let mut img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);


                        let mut sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                    image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                        let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                        c2.add_child(container);
                        i += 1;
                        pixels_vec.clear();
                    } else {
                        let mut _string;

                        let mut appStr = element.to_string();

                        if appStr.len() >= 1 {
                            if appStr.chars().last().unwrap() == '<' {
                                appStr.replace_range(appStr.len() - 1.., "");
                            }
                        }

                        _string = strip_tags(appStr.as_str());


                        let rl = Label::new(_string.clone())
                            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                        c2.add_child(rl);
                    }
                }
            }
        }
    }

    if state.double_page {
        let mut c3 = Flex::row();
        c3.add_flex_child(c.cross_axis_alignment(CrossAxisAlignment::Start), 1.0);
        c3.add_flex_child(c2.cross_axis_alignment(CrossAxisAlignment::Start), 1.0);

        scroll = Scroll::new(c3.cross_axis_alignment(CrossAxisAlignment::Start)).vertical();
    } else {
        scroll = Scroll::new(c.cross_axis_alignment(CrossAxisAlignment::Start)).vertical();
    }


    let padding = Padding::new((50.0, 10.), scroll);
    SizedBox::new(padding).expand_height().boxed()
}
