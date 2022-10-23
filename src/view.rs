#![allow(non_snake_case)]

use druid::{widget::{Flex}, Widget, WidgetExt, Color, UnitPoint, FileDialogOptions, FileSpec, ImageBuf, KeyOrValue, TextAlignment};
use druid::piet::ImageFormat;
use crate::data::*;
use druid::widget::{TextBox, Button, Scroll, SizedBox, Image, FillStrat, Label, CrossAxisAlignment, LineBreaking, Padding, Click, ControllerHost};
use voca_rs::strip::strip_tags;
use voca_rs::Voca;

//Creating the layout for defining a new bookmark

fn bookmark_row() -> impl Widget<AppState> {
    let mut r = Flex::row();

    r.add_child(Label::new("Create a new Bookmark").with_text_color(KeyOrValue::Concrete(Color::BLACK)).padding(5.0));
    r.add_child(TextBox::new().with_placeholder("Title")
        .padding(5.0)
        .lens(AppState::string_bookmark));

    r.add_child(Button::new("Apply").padding(5.0).on_click(AppState::click_confirm_bookmark_button));
    r.add_child(Button::new("Deny").padding(5.0).on_click(AppState::click_reject_bookmark_button));


    r.expand_width()
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

/*
Creating the layout for the functions: Open, Edit, Scan, Help and the page navigation section;
Open function is managed through FileDialogOptions;
TextBox in the navigation section depends on the state of the AppState's variable edit_current_page
and is done through the method lens.
*/

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
    let scan_button = Button::new("Scan").padding(5.0).on_click(AppState::click_scan_button);


    let help_button = Button::new("Help").padding(5.0).on_click(AppState::click_help_button);
    let previous_button = Button::new("Previous Page").padding(5.0).on_click(AppState::click_previous_button);
    let change_page = TextBox::new()
        .with_placeholder("50")
        .lens(AppState::edit_current_page);
    let next_button = Button::new("Next Page").padding(5.0).on_click(AppState::click_next_button);

    let r1 = Flex::row()
        .with_child(open_button)
        .with_child(edit_button)
        .with_child(scan_button)
        .with_child(help_button)
        .align_left();

    let r2 = Flex::row()
        .with_child(previous_button)
        .with_child(change_page)
        .with_child(next_button)
        .align_right();


    Flex::row()
        .with_flex_child(r1, 1.0)
        .with_flex_child(r2, 1.0)
        .expand_width()
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

/*
Creating the layout for the functions: Menu, Single Page, Double Page and the font size section;
The function Menu updates the state of the AppState's variable display_menu in order to edit the app
main section adding a new column;
TextBox in the font size section depends on the state of the AppState's variable font_size and is
done through the method lens.
*/
fn settings_row() -> impl Widget<AppState> {
    let display_menu_button = Button::new("Menu").padding(5.0).on_click(AppState::click_display_menu_button);
    let single_page_button = Button::new("Single Page").padding(5.0).on_click(AppState::click_single_page_button);
    let double_page_button = Button::new("Double Page").padding(5.0).on_click(AppState::click_double_page_button);
    let plus_button = Button::new("+").padding(5.0).on_click(AppState::click_plus_button);
    let min_button = Button::new("-").padding(5.0).on_click(AppState::click_min_button);


    let edit_size_text = TextBox::new()
        .with_placeholder("50")
        .lens(AppState::font_size);


    let r1 = Flex::row().with_child(display_menu_button).align_left();
    let r2 = Flex::row()
        .with_child(single_page_button)
        .with_child(double_page_button)
        .with_child(min_button)
        .with_child(edit_size_text)
        .with_child(plus_button)
        .align_right();

    Flex::row()
        .with_flex_child(r1, 1.0)
        .with_flex_child(r2, 1.0)
        .align_vertical(UnitPoint::BOTTOM)
        .expand_width()
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

/*
This is the main function of the whole application, it is invocated by the main and its purpose is
to create the entire User Interface.
*/
pub fn build_ui() -> impl Widget<AppState> {
    let mut c = Flex::column();
    c.add_child(option_row());
    c.add_child(bookmark_row());
    c.add_flex_child(Rebuilder::new(), 1.0);
    c.add_child(settings_row());
    return c;
}

/*
This function is invocated when the user press the Edit button and all the constraints are met.
Its purpose is to build a new view based on the previous one, but with different functionalities
*/
pub fn build_ui_edit_mode() -> impl Widget<AppState> {
    let mut c = Flex::column();

    c.add_child(option_row_edit_mode());

    c.add_flex_child(Scroll::new(TextBox::multiline().lens(AppState::current_page_text).expand_width()).vertical(), 1.);

    return c;
}

/*
Creating the layout for the functions in the edit view: Save new version and Undo;
The function Save new version submits the command SHOW_SAVE_PANEL;
The function Undo submits a new command with the Selector MODIFY_EDIT_MODE and also the command
CLOSE_WINDOW.
*/
fn option_row_edit_mode() -> impl Widget<AppState> {
    let epub = FileSpec::new("Epub file", &["epub"]);
    let default_save_name = String::from("MyFile.epub");

    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![epub])
        .default_type(epub)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target for this lovely file")
        .button_text("Export");


    let save_button = Button::new("Save new version").padding(5.0).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()));
    });

    let undo_button = Button::new("Undo").padding(5.0).on_click(move |ctx, _, _| {
        ctx.submit_command(MODIFY_EDIT_MODE.with(false));
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
    });


    let r1 = Flex::row()
        .with_child(save_button)
        .with_child(undo_button)
        .align_left();

    Flex::row()
        .with_flex_child(r1, 1.0)
        .expand_width()
        .background(Color::WHITE)
        .border(Color::GRAY, 0.5)
}

/*
This function checks if a passed String is parsable into a f64, if so it also checks if the number
is positive or not. The function returns "Ok" if it passes all the checks.
*/
pub fn check_valid_number(elem: String) -> String {
    if elem.parse::<f64>().is_ok() {
        if elem.parse::<f64>().unwrap().is_sign_positive() {
            return "Ok".to_string();
        } else {
            return "Not valid".to_string();
        }
    } else {
        return "Not valid".to_string();
    }
}

/*
This is the core of the main section of the view, it builds a Box of dynamic Widgets of AppState.
Its purpose is to display the ebook whenever is selected and valid; there are 4 versions for this
view: Single Page, Double Page and these 2 versions with the menu open. It also creates a Row
indicating the current page/s and the size of the Label depends on the font_size AppState variable.
In case there is no Ebook selected, there will be a default Label in the main view to describe to
the user how to start using the app.
*/
pub fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    let mut c = Flex::column();
    let mut i = 0 as usize;
    let mut pixels_vec = Vec::new();
    let mut image_buf;
    let scroll;
    let mut c2 = Flex::column();

    if state.ebook.len() <= 0 {
        c.add_child(Label::new("\n\n\n\n\n\n\n\n\t\t\t\t\t\t\t\t\tWelcome to Ebook Reader!\n\t\t\t\t\t\t\tPress the Open button to start reading an Ebook\n\t\t\t\t\t\t\tUse Help button to open the application guide")
            .with_text_size(KeyOrValue::Concrete(20.)));
    } else if state.ebook.len() > 0
        && state.font_size != "0"
        && state.edit_current_page.len() > 0
        && check_valid_number(state.clone().edit_current_page) != "Not valid"
        && state.edit_current_page._is_numeric() {
        if state.current_page != 0 {
            let mut str_page_number = String::new();
            str_page_number.push_str((state.current_page).to_string().as_str());
            str_page_number.push_str("\n\n");

            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                let rl_page = Label::new(str_page_number)
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_text_alignment(TextAlignment::Center)
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                c.add_child(rl_page);
            } else {
                let rl_page = Label::new(str_page_number)
                    .with_text_size(KeyOrValue::Concrete(1.))
                    .with_text_alignment(TextAlignment::Center)
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                c.add_child(rl_page);
            }
        }


        let init = state.ebook[state.current_page].text.find("<body");
        let end_body = state.ebook[state.current_page].text.find("</html>");

        if end_body.is_some() && init.is_some() {
            if end_body.unwrap() < init.unwrap() {
                for element in state.ebook[state.current_page].text[..end_body.unwrap()].split("\n") {
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


                        let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                        if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                            let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                    image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                            let container = sized.border(Color::grey(0.6), 2.0).center().boxed();
                            c.add_child(container);
                        } else {
                            let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                    image_buf.height().clone() as f64 * (1. / 40.));

                            let container = sized.border(Color::grey(0.6), 2.0).center().boxed();
                            c.add_child(container);
                        }


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

                        if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                            let rl = Label::new(_string.clone())
                                .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);
                            c.add_child(rl);
                        } else {
                            let rl = Label::new(_string.clone())
                                .with_text_size(KeyOrValue::Concrete(1.))
                                .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);
                            c.add_child(rl);
                        }
                    }
                }
            }
        }

        if init.is_some() {
            for element in state.ebook[state.current_page].text[init.unwrap()..].split("\n") {
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


                    let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                        let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                        c.add_child(container);
                    } else {
                        let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                image_buf.height().clone() as f64 * (1. / 40.));

                        let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                        c.add_child(container);
                    }


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

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let rl = Label::new(_string.clone())
                            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                        c.add_child(rl);
                    } else {
                        let rl = Label::new(_string.clone())
                            .with_text_size(KeyOrValue::Concrete(1.))
                            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                        c.add_child(rl);
                    }
                }
            }
        } else {
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


                    let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                        let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                        c.add_child(container);
                    } else {
                        let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                image_buf.height().clone() as f64 * (1. / 40.));

                        let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                        c.add_child(container);
                    }


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

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let rl = Label::new(_string.clone())
                            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                        c.add_child(rl);
                    } else {
                        let rl = Label::new(_string.clone())
                            .with_text_size(KeyOrValue::Concrete(1.))
                            .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                        c.add_child(rl);
                    }
                }
            }
        }


        if state.double_page {
            if state.current_page + 1 < state.ebook.len() {
                let mut str_page = String::new();
                str_page.push_str((state.current_page + 1).to_string().as_str());
                str_page.push_str("\n\n");
                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                    let rl_page = Label::new(str_page)
                        .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                        .with_text_alignment(TextAlignment::Center)
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                    c2.add_child(rl_page);
                } else {
                    let rl_page = Label::new(str_page)
                        .with_text_size(KeyOrValue::Concrete(1.))
                        .with_text_alignment(TextAlignment::Center)
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                    c2.add_child(rl_page);
                }

                let init_double = state.ebook[state.current_page + 1].text.find("<body");
                let end_body_double = state.ebook[state.current_page + 1].text.find("</html>");
                i = 0;
                if init_double.is_some() && end_body_double.is_some() {
                    if end_body_double.unwrap() < init_double.unwrap() {
                        for element in state.ebook[state.current_page + 1].text[..end_body_double.unwrap()].split("\n") {
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


                                let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                    let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                            image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                                    let container = sized.border(Color::grey(0.6), 2.0).center().boxed();
                                    c2.add_child(container);
                                } else {
                                    let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                            image_buf.height().clone() as f64 * (1. / 40.));

                                    let container = sized.border(Color::grey(0.6), 2.0).center().boxed();
                                    c2.add_child(container);
                                }


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

                                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                    let rl = Label::new(_string.clone())
                                        .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);
                                    c2.add_child(rl);
                                } else {
                                    let rl = Label::new(_string.clone())
                                        .with_text_size(KeyOrValue::Concrete(1.))
                                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);
                                    c2.add_child(rl);
                                }
                            }
                        }
                    }
                }
                if init_double.is_some() {
                    for element in state.ebook[state.current_page + 1].text[init_double.unwrap()..].split("\n") {
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


                            let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                        image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                                c2.add_child(container);
                            } else {
                                let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                        image_buf.height().clone() as f64 * (1. / 40.));

                                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                                c2.add_child(container);
                            }


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

                            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                let rl = Label::new(_string.clone())
                                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                                c2.add_child(rl);
                            } else {
                                let rl = Label::new(_string.clone())
                                    .with_text_size(KeyOrValue::Concrete(1.))
                                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                                c2.add_child(rl);
                            }
                        }
                    }
                } else {
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


                            let img = Image::new(image_buf.clone()).fill_mode(FillStrat::Fill);

                            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.),
                                                                        image_buf.height().clone() as f64 * (state.font_size.clone().parse::<f64>().unwrap() / 40.));

                                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                                c2.add_child(container);
                            } else {
                                let sized = SizedBox::new(img).fix_size(image_buf.width().clone() as f64 * (1. / 40.),
                                                                        image_buf.height().clone() as f64 * (1. / 40.));

                                let container = sized.border(Color::grey(0.6), 2.0).center().boxed();

                                c2.add_child(container);
                            }


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

                            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                                let rl = Label::new(_string.clone())
                                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                                c2.add_child(rl);
                            } else {
                                let rl = Label::new(_string.clone())
                                    .with_text_size(KeyOrValue::Concrete(1.))
                                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size);

                                c2.add_child(rl);
                            }
                        }
                    }
                }
            }
        }
    }

    if state.double_page {
        let mut c3 = Flex::row();
        if state.display_menu && state.font_size != "0" {
            let mut c4 = Flex::column();

            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                c4.add_child(Padding::new((0., 10.), Label::new("BOOKMARKS")
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            } else {
                c4.add_child(Padding::new((0., 10.), Label::new("BOOKMARKS")
                    .with_text_size(KeyOrValue::Concrete(1.))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            }

            if state.saves.bookmarks.len() > 0 {
                for bookmark in state.saves.bookmarks.clone() {
                    let mut ro = Flex::row();

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let ch = ControllerHost::new(Label::new(bookmark.0.clone() + " - pag. " + (bookmark.1.clone()).to_string().as_str())
                                                         .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                            ctx.submit_command(GO_TO_POS.with(bookmark.1.clone()));
                        }));

                        ro.add_flex_child(ch, 1.0);
                    } else {
                        let ch = ControllerHost::new(Label::new(bookmark.0.clone() + " - pag. " + (bookmark.1.clone()).to_string().as_str())
                                                         .with_text_size(KeyOrValue::Concrete(1.))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                            ctx.submit_command(GO_TO_POS.with(bookmark.1.clone()));
                        }));

                        ro.add_flex_child(ch, 1.0);
                    }


                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let x_button = Label::new("x")
                            .with_text_color(KeyOrValue::Concrete(Color::RED))
                            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                            .on_click(move |ctx, _, _| {
                                ctx.submit_command(DELETE_BOOKMARK.with(bookmark.clone()));
                            });

                        ro.add_flex_child(x_button, 0.6);
                        c4.add_child(ro);
                    } else {
                        let x_button = Label::new("x")
                            .with_text_color(KeyOrValue::Concrete(Color::RED))
                            .with_text_size(KeyOrValue::Concrete(1.))
                            .on_click(move |ctx, _, _| {
                                ctx.submit_command(DELETE_BOOKMARK.with(bookmark.clone()));
                            });

                        ro.add_flex_child(x_button, 0.6);
                        c4.add_child(ro);
                    }
                }
            } else {
                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                    c4.add_child(Padding::new((0., 10.), Label::new("No bookmarks available")
                        .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
                } else {
                    c4.add_child(Padding::new((0., 10.), Label::new("No bookmarks available")
                        .with_text_size(KeyOrValue::Concrete(1.))
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
                }
            }

            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                c4.add_child(Padding::new((0., 20.), Label::new("CHAPTERS")
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            } else {
                c4.add_child(Padding::new((0., 20.), Label::new("CHAPTERS")
                    .with_text_size(KeyOrValue::Concrete(1.))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            }


            for chapter in state.chapters.clone() {
                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                    c4.add_child(ControllerHost::new(Label::new(chapter.title.clone())
                                                         .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                        ctx.submit_command(GO_TO_POS.with(chapter.target_page.clone()));
                    })));
                    c4.add_child(Label::new("\n"));
                } else {
                    c4.add_child(ControllerHost::new(Label::new(chapter.title.clone())
                                                         .with_text_size(KeyOrValue::Concrete(1.))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                        ctx.submit_command(GO_TO_POS.with(chapter.target_page.clone()));
                    })));
                    c4.add_child(Label::new("\n"));
                }
            }
            c3.add_flex_child(c4, 0.2);
            let padd = Padding::new((30., 0.), c.cross_axis_alignment(CrossAxisAlignment::Start));
            c3.add_flex_child(padd, 1.0);
        } else {
            c3.add_flex_child(c.cross_axis_alignment(CrossAxisAlignment::Start), 1.0);
        }

        let padd2 = Padding::new((30., 0.), c2.cross_axis_alignment(CrossAxisAlignment::Start));
        c3.add_flex_child(padd2, 1.0);
        scroll = Scroll::new(c3.cross_axis_alignment(CrossAxisAlignment::Start)).vertical();
    } else {
        let mut c3 = Flex::row();
        if state.display_menu && state.font_size != "0" {
            let mut c4 = Flex::column();

            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                c4.add_child(Padding::new((0., 10.), Label::new("BOOKMARKS")
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            } else {
                c4.add_child(Padding::new((0., 10.), Label::new("BOOKMARKS")
                    .with_text_size(KeyOrValue::Concrete(1.))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            }

            if state.saves.bookmarks.len() > 0 {
                for bookmark in state.saves.bookmarks.clone() {
                    let mut ro = Flex::row();

                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let ch = ControllerHost::new(Label::new(bookmark.0.clone() + " - pag. " + (bookmark.1.clone()).to_string().as_str())
                                                         .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                            ctx.submit_command(GO_TO_POS.with(bookmark.1.clone()));
                        }));

                        ro.add_flex_child(ch, 1.0);
                    } else {
                        let ch = ControllerHost::new(Label::new(bookmark.0.clone() + " - pag. " + (bookmark.1.clone() + 1).to_string().as_str())
                                                         .with_text_size(KeyOrValue::Concrete(1.))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                            ctx.submit_command(GO_TO_POS.with(bookmark.1.clone()));
                        }));

                        ro.add_flex_child(ch, 1.0);
                    }


                    if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                        let x_button = Label::new("x")
                            .with_text_color(KeyOrValue::Concrete(Color::RED))
                            .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                            .on_click(move |ctx, _, _| {
                                ctx.submit_command(DELETE_BOOKMARK.with(bookmark.clone()));
                            });

                        ro.add_flex_child(x_button, 0.5);
                        c4.add_child(ro);
                    } else {
                        let x_button = Label::new("x")
                            .with_text_color(KeyOrValue::Concrete(Color::RED))
                            .with_text_size(KeyOrValue::Concrete(1.))
                            .on_click(move |ctx, _, _| {
                                ctx.submit_command(DELETE_BOOKMARK.with(bookmark.clone()));
                            });

                        ro.add_flex_child(x_button, 0.5);
                        c4.add_child(ro);
                    }
                }
            } else {
                if state.font_size.len() > 0 && state.font_size.clone().parse::<f64>().is_ok() {
                    c4.add_child(Padding::new((0., 10.), Label::new("No bookmarks available")
                        .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
                } else {
                    c4.add_child(Padding::new((0., 10.), Label::new("No bookmarks available")
                        .with_text_size(KeyOrValue::Concrete(1.))
                        .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
                }
            }

            if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                c4.add_child(Padding::new((0., 20.), Label::new("CHAPTERS")
                    .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            } else {
                c4.add_child(Padding::new((0., 10.), Label::new("No bookmarks available")
                    .with_text_size(KeyOrValue::Concrete(1.))
                    .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size)));
            }


            for chapter in state.chapters.clone() {
                if state.font_size.len() > 0 && check_valid_number(state.font_size.clone()) == "Ok" {
                    c4.add_child(ControllerHost::new(Label::new(chapter.title.clone())
                                                         .with_text_size(KeyOrValue::Concrete(state.font_size.clone().parse::<f64>().unwrap()))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                        ctx.submit_command(GO_TO_POS.with(chapter.target_page.clone()));
                    })));

                    c4.add_child(Label::new("\n"));
                } else {
                    c4.add_child(ControllerHost::new(Label::new(chapter.title.clone())
                                                         .with_text_size(KeyOrValue::Concrete(1.))
                                                         .with_text_color(KeyOrValue::Concrete(Color::LIME))
                                                         .with_line_break_mode(LineBreaking::WordWrap).fix_width(state.window_size), Click::new(move |ctx, _, _| {
                        ctx.submit_command(GO_TO_POS.with(chapter.target_page.clone()));
                    })));

                    c4.add_child(Label::new("\n"));
                }
            }
            c3.add_flex_child(c4, 0.2);
            let padd = Padding::new((20., 0.), c.cross_axis_alignment(CrossAxisAlignment::Start));
            c3.add_flex_child(padd, 1.0);
        } else {
            c3.add_flex_child(c.cross_axis_alignment(CrossAxisAlignment::Start), 1.0);
        }

        scroll = Scroll::new(c3.cross_axis_alignment(CrossAxisAlignment::Start)).vertical();
    }


    let padding = Padding::new((50.0, 10.), scroll);
    SizedBox::new(padding).expand_height().boxed()
}
