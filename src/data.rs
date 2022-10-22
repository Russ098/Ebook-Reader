#![allow(deprecated)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::{fs, io};
use druid::{Data, Lens, EventCtx, Env, commands, AppDelegate, DelegateCtx, Target, Command, Handled, Widget, WidgetExt, Event, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, WidgetId, Selector, WindowDesc, WindowId, FileDialogOptions, FileSpec};
use std::fs::{File};
use std::io::{BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};
use druid::im::Vector;
use druid::widget::{SizedBox};
use epub::archive::EpubArchive;
use imagesize::{blob_size};
use native_dialog::{MessageDialog, MessageType};
use voca_rs::Voca;
use crate::view::{build_ui_edit_mode, build_widget};
use serde::Serialize;
use serde::Deserialize;
use voca_rs::strip::strip_tags;
use zip::{CompressionMethod};
use zip::result::ZipError;
use zip::write::FileOptions;
use walkdir::{WalkDir, DirEntry as OtherDirEntry};


const SIZE_FONT: f64 = 20.0;

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct ImageOfPage {
    pub image: Vector<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageOfPage {
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
        if data.window_size != self.window_size {
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
            if data.ebook.len() > 0 {
                if old_data.edit_current_page != data.edit_current_page {
                    if data.edit_current_page.parse::<usize>().is_ok() {
                        if data.edit_current_page.parse::<usize>().unwrap() < data.ebook.len() {
                            if data.edit_mode == false {
                                if data.edit_current_page.len() == 0 {
                                    ctx.submit_command(GO_TO_POS_FROM_EDIT.with(1));
                                } else {
                                    ctx.submit_command(GO_TO_POS_FROM_EDIT.with(data.edit_current_page.parse::<usize>().unwrap()));
                                }
                            }
                        } else {
                            if data.edit_mode == false {
                                ctx.submit_command(GO_TO_POS_FROM_EDIT.with(data.ebook.len() - 1));
                            }
                        }
                    }
                }
            }
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

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
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
}

#[derive(Clone, Data, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub target_page: usize,
}

impl Chapter {
    pub fn from(title: String, page: usize) -> Self {
        Self {
            title: String::from(title),
            target_page: page,
        }
    }
}

#[derive(Clone, Data, Serialize, Deserialize)]
pub struct JsonStruct {
    pub bookmarks: Vector<(String, usize)>,
    pub last_page: usize,
}

impl JsonStruct {
    pub fn new() -> Self {
        Self {
            bookmarks: Vector::new(),
            last_page: 0,
        }
    }
}


#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct AppState {
    pub font_size: String,
    pub ebook: Vector<Page>,
    pub current_page: usize,
    pub window_size: f64,
    pub double_page: bool,
    pub title: String,
    pub chapters: Vector<Chapter>,
    pub saves: JsonStruct,
    pub edit_mode: bool,
    pub display_menu: bool,
    pub new_bookmark: bool,
    pub string_bookmark: String,
    pub current_page_text: String,
    pub file_info: String,
    pub scan_mode: bool,
    pub edit_current_page: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            ebook: Vector::<Page>::new(),
            current_page: 0,
            window_size: 1100.,
            double_page: false,
            title: String::new(),
            chapters: Vector::<Chapter>::new(),
            saves: JsonStruct::new(),
            edit_mode: false,
            display_menu: false,
            new_bookmark: false,
            string_bookmark: String::new(),
            current_page_text: String::new(),
            file_info: String::new(),
            scan_mode: false,
            edit_current_page: String::from("0"),

        }
    }

    pub fn click_plus_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.plus();
    }
    fn plus(&mut self) {
        let new_size = self.font_size.parse::<f64>().unwrap() + 1.;
        self.font_size = new_size.to_string();
    }
    pub fn click_min_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.min();
    }
    fn min(&mut self) {
        if self.font_size.parse::<f64>().unwrap() > 0. {
            let new_size = self.font_size.parse::<f64>().unwrap() - 1.;
            self.font_size = new_size.to_string();
        }
    }
    pub fn click_edit_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while trying to open an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while showing warning when the app is in EDIT MODE");
        } else {
            data.edit_mode = !data.edit_mode;

            data.current_page_text = data.ebook[data.current_page].clone().text;


            let new_win = WindowDesc::new(build_ui_edit_mode)
                .title("Edit Ebook")
                .window_size(Size::new(1200., 700.));

            _ctx.new_window(new_win);
        }
    }


    pub fn click_scan_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode == true {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook and the app is in EDIT MODE");
        } else {
            data.scan_mode = true;

            let jpg = FileSpec::new("Jpg file", &["jpg"]);
            let jpeg = FileSpec::new("Jpeg file", &["jpeg"]);
            let png = FileSpec::new("Png file", &["png"]);
            let default_save_name = String::from("MyFile.jpg");

            let save_dialog_options = FileDialogOptions::new()
                .allowed_types(vec![jpg, jpeg, png])
                .default_type(jpg)
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

            _ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()));
        }
    }

    pub fn click_single_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            data.double_page = false;
        }
    }

    pub fn click_double_page_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            data.double_page = true;
        }
    }


    pub fn click_help_button(_ctx: &mut EventCtx, _: &mut Self, _env: &Env) {

        let help_description = String::from("Welcome to Ebook Reader application v1.0.0, this is a short guide for the application.
        \nThese are the functions that you can use:
        \n- Open: use this button to open an epub file and navigate through its content
        \n- Edit: use this button to edit the epub content of the current page. Be careful, inappropriate html changes can corrupt the new file, we suggest you to only edit the contents of the tags
        \n- Scan: use this button to select a jpg/png image of the physical book to navigate to the corresponding digital page. Be careful, the success of this function depends on the image quality!
        \n- Previous/Next Page: use these buttons to navigate through the ebook digital pages, you can use the text field to directly navigate to the desired page
        \n- Bookmark section: this section allows you to create a new bookmark at the current page assigning it a name
        \n- Menu: use this button to toggle a new section in which you can find all the bookmarks you created and ebook chapters, you can interact with them (by clicking) to navigate to the corresponding page. To delete a bookmark click on the red 'x'
        \n- Single Page/Double Page: use these buttons to display one or two digital pages
        \n- Font Section: use this section to adjust the text dimensions");

        let dialog = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_text(help_description.clone().as_str())
            .set_title("Help")
            .show_alert()
            .expect("Error opening help dialog");
    }

    pub fn click_previous_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            if data.double_page {
                if data.current_page > 1 {
                    data.current_page -= 2;
                    let new_page = data.current_page.to_string();
                    data.edit_current_page = new_page;
                    data.saves.last_page = data.current_page;
                    data.save_to_json();
                }
            } else {
                if data.current_page > 0 {
                    data.current_page -= 1;
                    let new_page = data.current_page.to_string();
                    data.edit_current_page = new_page;
                    data.saves.last_page = data.current_page;
                    data.save_to_json();
                }
            }
        }
    }

    pub fn click_next_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            if data.double_page {
                if data.current_page < (data.ebook.len() - 2) {
                    data.current_page += 2;
                    let new_page = data.current_page.to_string();
                    data.edit_current_page = new_page;
                    data.saves.last_page = data.current_page;
                    data.save_to_json();
                }
            } else {
                if data.current_page < (data.ebook.len() - 1) {
                    data.current_page += 1;
                    let new_page = data.current_page.to_string();
                    data.edit_current_page = new_page;
                    data.saves.last_page = data.current_page;
                    data.save_to_json();
                }
            }
        }
    }

    pub fn click_display_menu_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            data.display_menu = !data.display_menu;
        }
    }

    pub fn click_confirm_bookmark_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            if data.string_bookmark.len() == 0 {
                MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_text("Please insert a title in order to create a new bookmark")
                    .set_title("No title for the bookmark")
                    .show_alert().expect("Error while trying to load bookmarks");
                return;
            }


            let mut found = false;
            for bookmark in data.saves.bookmarks.clone() {
                if bookmark.1 == data.current_page {
                    found = true;
                }
            }

            if found {
                MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_text("You've already inserted a bookmark for this page")
                    .set_title("Bookmark already inserted")
                    .show_alert().expect("Error while founding the corresponding digital page");
            } else {
                data.saves.bookmarks.push_back((data.string_bookmark.clone(), data.current_page.clone()));
                data.save_to_json();
                data.string_bookmark = String::new();
                data.new_bookmark = false;
            }
        }
    }

    pub fn click_reject_bookmark_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.ebook.len() == 0 {
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_text("Please select an Ebook to enable this function.")
                .set_title("Ebook not selected")
                .show_alert().expect("Error while selecting an Ebook");
        } else if data.edit_mode {
            MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                .set_title("Ebook in edit mode")
                .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
        } else {
            data.string_bookmark = String::new();
            data.new_bookmark = false;
        }
    }


    pub fn save_to_json(&self) {
        let serialized = serde_json::to_string(&self.saves).unwrap();

        let filename = self.title.clone() + ".json";
        let path = Path::new("\\Ebook_Reader\\Metadata\\");

        std::fs::create_dir_all(path).unwrap();

        let mut p = String::from(path.to_str().unwrap());
        p.push_str(filename.as_str());
        std::fs::write(p, serialized).unwrap();
    }

    pub fn load_from_json(&mut self) {
        let mut path = String::from("\\Ebook_Reader\\Metadata\\");
        path.push_str(self.title.as_str());
        path.push_str(".json");
        let file = File::open(path);

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut de = serde_json::Deserializer::from_reader(reader);
                let u = JsonStruct::deserialize(&mut de).unwrap();

                self.saves = u;
                self.current_page = self.saves.last_page;

                let new_page = self.current_page.to_string();
                self.edit_current_page = new_page;
            }
            Err(_) => {}
        }
    }
}

fn zip_dir<T>(it: &mut dyn Iterator<Item=OtherDirEntry>, prefix: &str, writer: T, method: zip::CompressionMethod)
              -> zip::result::ZipResult<()>
    where T: Write + Seek
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip

            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn doit(src_dir: &str, dst_file: &str, method: zip::CompressionMethod) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}


pub const GO_TO_POS: Selector<usize> = Selector::new("go_to_pos");
pub const DELETE_BOOKMARK: Selector<(String, usize)> = Selector::new("delete_bookmark");
pub const MODIFY_EDIT_MODE: Selector<bool> = Selector::new("modify_edit_mode");
pub const GO_TO_POS_FROM_EDIT: Selector<usize> = Selector::new("edit_current_page");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn window_removed(
        &mut self,
        _: WindowId,
        data: &mut AppState,
        _: &Env,
        _: &mut DelegateCtx<'_>,
    ) {
        data.edit_mode = false;
    }


    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if Path::new(file_info.path().to_str().unwrap()).exists() {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_text("There is already an ebook with this name in this folder, try again with another name or change folder.")
                    .set_title("Ebook already exists")
                    .show_alert().expect("Error while trying to save file");
            } else {
                // let f = File::create(file_info.path().to_str().unwrap());
                fs::copy(data.file_info.clone(), Path::new(file_info.path().to_str().unwrap())).expect("Error while trying to copy file");

                let path = PathBuf::from(file_info.path().to_str().unwrap());
                let dest_path = Path::new("/Ebook_Reader/output/");


                fs::create_dir(dest_path).unwrap();
                fs::create_dir(Path::new("/Ebook_Reader/output/META-INF")).unwrap();
                fs::create_dir(Path::new("/Ebook_Reader/output/OEBPS")).unwrap();

                let fname = std::path::Path::new(&path);
                let file = fs::File::open(&fname).unwrap();

                let mut archive = zip::ZipArchive::new(file).unwrap();

                let mut current_chapter = 0;
                let mut start_page_chapter = 0;
                let mut last_initial_page = 0;
                let mut stop_page = 0;
                let mut found_start_page = false;
                let mut already_found = false;
                let mut i_found = 0;
                let mut next_stop_page = 0;

                data.chapters.iter().enumerate().for_each(|(i, x)| {
                    if i == i_found {
                        next_stop_page = x.target_page;
                    }

                    if found_start_page && stop_page == 0 {
                        stop_page = x.target_page;
                        found_start_page = false;
                    }
                    if x.target_page > data.current_page && !already_found {
                        current_chapter = i - 2;
                        start_page_chapter = last_initial_page;
                        stop_page = x.target_page;
                        found_start_page = true;
                        already_found = true;
                        i_found = i + 1;

                        if !data.ebook[start_page_chapter].text.contains("<?xml") {
                            start_page_chapter -= 1;
                        }
                    } else if x.target_page == data.current_page && !already_found {
                        current_chapter = i - 1;
                        start_page_chapter = x.target_page;
                        found_start_page = true;
                        already_found = true;
                        i_found = i + 1;

                        if !data.ebook[start_page_chapter].text.contains("<?xml") {
                            start_page_chapter -= 1;
                        }
                    }

                    last_initial_page = x.target_page;
                });

                let mut new_content = String::new();
                let mut next_content = String::new();
                let mut one_page = false;

                if start_page_chapter == (stop_page - 1) {
                    one_page = true;
                    new_content.push_str(data.current_page_text.as_str());
                } else {
                    for i in start_page_chapter..stop_page {
                        if data.current_page == i {
                            if i == start_page_chapter {
                                new_content.push_str(&data.current_page_text[data.current_page_text.find("<?xml").unwrap()..]);
                            } else if i == stop_page - 1 {
                                new_content.push_str(&data.current_page_text[..data.current_page_text.find("</html>").unwrap() + 7]);
                                for j in i..next_stop_page {
                                    if j == i {
                                        next_content.push_str(&data.current_page_text[data.current_page_text.find("<?xml").unwrap()..]);
                                    } else if j == next_stop_page - 1 {
                                        next_content.push_str(&data.ebook[j].text[..data.ebook[j].text.find("</html>").unwrap() + 7]);
                                    } else {
                                        next_content.push_str(data.ebook[j].text.as_str());
                                    }
                                }
                            } else {
                                new_content.push_str(data.current_page_text.as_str());
                            }
                        } else {
                            if i == start_page_chapter {
                                new_content.push_str(&data.ebook[i].text[data.ebook[i].text.find("<?xml").unwrap()..]);
                            } else if i == stop_page - 1 {
                                new_content.push_str(&data.ebook[i].text[..data.ebook[i].text.find("</html>").unwrap() + 7]);
                            } else {
                                new_content.push_str(data.ebook[i].text.as_str());
                            }
                        }
                    }
                }

                let mut file_to_find = "h-".to_string();
                file_to_find.push_str(current_chapter.to_string().as_str());
                file_to_find.push_str(".htm.html");

                let mut file_to_find_next = "h-".to_string();
                file_to_find_next.push_str((current_chapter + 1).to_string().as_str());
                file_to_find_next.push_str(".htm.html");

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = match file.enclosed_name() {
                        Some(path) => { path.to_owned() }
                        None => continue,
                    };


                    if (file.name()).ends_with('/') {
                        fs::create_dir_all(&outpath).unwrap();
                    } else {
                        if file.name().contains(&file_to_find.clone()) {
                            file_to_find = file.name().to_string();
                        }

                        if file.name().contains(&file_to_find_next.clone()) {
                            file_to_find_next = file.name().to_string();
                        }


                        if let Some(p) = outpath.parent() {
                            if !p.exists() {
                                let mut a = dest_path.to_str().unwrap().to_string();
                                a.push_str(p.to_str().unwrap());
                                fs::create_dir_all(a).unwrap();
                            }
                        }
                        let mut a2 = dest_path.to_str().unwrap().to_string();
                        a2.push_str(outpath.to_str().unwrap());
                        let mut outfile = fs::File::create(a2).unwrap();
                        io::copy(&mut file, &mut outfile).unwrap();
                    }
                }

                if data.current_page == stop_page - 1 && !one_page {
                    let mut file_to_edit = dest_path.to_str().unwrap().to_string();
                    file_to_edit.push_str(file_to_find_next.as_str());
                    File::create(file_to_edit.clone()).expect("Error while trying to create file to be edited");

                    let mut f2 = std::fs::OpenOptions::new().write(true).truncate(true).open(file_to_edit).unwrap();
                    f2.write_all(next_content.as_bytes()).unwrap();
                    f2.flush().unwrap();
                }

                let mut file_to_edit = dest_path.to_str().unwrap().to_string();
                file_to_edit.push_str(file_to_find.as_str());
                File::create(file_to_edit.clone()).expect("Error while trying to create file to be edited");

                let mut f2 = std::fs::OpenOptions::new().write(true).truncate(true).open(file_to_edit).unwrap();
                f2.write_all(new_content.as_bytes()).unwrap();
                f2.flush().unwrap();


                doit(dest_path.to_str().unwrap(), file_info.path().to_str().unwrap(), CompressionMethod::Bzip2).expect("Error while trying to zip file");
                let mut str = "File correctly saved at: ".to_string();
                str.push_str(file_info.path().to_str().unwrap());

                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_text(str.as_str())
                    .set_title("Success")
                    .show_alert().expect("Error while saving file");

                fs::remove_dir_all(dest_path).expect("Error while trying to remove directory");
            }
        }

        if data.scan_mode {
            if let Some(file_info_scan) = cmd.get(commands::OPEN_FILE) {
                let output = if cfg!(target_os = "windows") {
                    std::process::Command::new("tesseract")
                        .args([file_info_scan.path().to_str().unwrap(), "stdout"])
                        .output()
                        .expect("failed to execute process")
                } else {
                    std::process::Command::new("tesseract")
                        .args([file_info_scan.path().to_str().unwrap(), "stdout"])
                        .output()
                        .expect("failed to execute process")
                };

                let result = output.stdout;
                let s = String::from_utf8(result).unwrap();

                let mut count = 0.;
                let mut percentage = 0.;
                let mut index = 0;

                data.ebook.iter().enumerate().for_each(|(i, x)| {
                    if i != 0 {
                        if x.text.len() > 0 {
                            let mut app = x.clone().text._strip_tags();
                            for elem in s.split_whitespace() {
                                let mut app2: String = " ".to_string();
                                app2.push_str(elem);
                                app2.push_str(" ");

                                if app.contains(&app2) && app.len() > 0 {
                                    count += 1.;
                                    app = app.replacen(&app2, "", 1);
                                }
                            }

                            let temp: f64 = count / (count + app.split_whitespace().count() as f64);
                            if temp > percentage {
                                percentage = temp;
                                index = i;
                            }
                            count = 0.;
                        }
                    }
                });

                if percentage > 0.1 {
                    data.current_page = index;
                    let mut res = "Research of the scanned page ended successfully, page found: ".to_string();
                    res.push_str(data.current_page.to_string().as_str());

                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_text(res.as_str())
                        .set_title("Page found")
                        .show_alert().expect("Error while founding corresponding digital page");
                } else {
                    let res = "Research of the scanned page was not successfull, accuracy is too low".to_string();

                    MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_text(res.as_str())
                        .set_title("Page not found")
                        .show_alert().expect("Error while discovering corresponing digital page is not successful");
                }
            }
        }

        if cmd.is(GO_TO_POS_FROM_EDIT) {
            let pos = cmd.get_unchecked(GO_TO_POS_FROM_EDIT);
            data.current_page = *pos;
            data.edit_current_page = (*pos.to_string()).to_string();
            data.save_to_json();
        }

        if cmd.is(MODIFY_EDIT_MODE) {
            let pos = cmd.get_unchecked(MODIFY_EDIT_MODE);
            data.edit_mode = *pos;
        }

        if cmd.is(GO_TO_POS) {
            let pos = cmd.get_unchecked(GO_TO_POS);
            data.current_page = *pos;
            let new_page = data.current_page.to_string();
            data.edit_current_page = new_page;
        }

        if cmd.is(DELETE_BOOKMARK) {
            let pos = cmd.get_unchecked(DELETE_BOOKMARK);
            let mut i = 0;
            for bookmark in data.saves.bookmarks.clone() {
                if bookmark.eq(pos) {
                    break;
                } else {
                    i += 1;
                }
            }

            data.saves.bookmarks.remove(i);
            data.save_to_json();
        }


        if !data.scan_mode {
            if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
                if data.edit_mode {
                    MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_text("There is an Ebook open in edit mode, close that window to use again this function.")
                        .set_title("Ebook in edit mode")
                        .show_alert().expect("Error while selecting an Ebook but the app is in EDIT MODE");
                    return Handled::Yes;
                }

                data.file_info = file_info.clone().path().to_str().unwrap().to_string();
                match EpubArchive::new(file_info.clone().path())
                {
                    Ok(mut archive) => {
                        if data.ebook.len() > 0 {
                            data.saves.last_page = data.current_page;
                            data.save_to_json();
                        }

                        data.ebook.clear();
                        data.current_page = 0;
                        data.edit_current_page = String::from("0");
                        data.title = file_info.clone().path().to_str().unwrap().split("\\")
                            .last().unwrap().split(".")
                            .next().unwrap().to_string();
                        data.saves.last_page = 0;
                        data.saves.bookmarks.clear();
                        data.display_menu = false;
                        data.edit_mode = false;
                        data.new_bookmark = false;
                        data.chapters.clear();

                        data.load_from_json();

                        let mut page_no = 1;
                        let mut page_not_ended = false;
                        let mut chapter_title: String = String::new();
                        let mut past_page_no = 0;


                        alphanumeric_sort::sort_path_slice(&mut archive.files);

                        for f in archive.files.clone() {
                            if f.contains("OEBPS") && (f.contains("htm.html") || f.contains("wrap")) {
                                data.ebook.push_back(Page::new());
                                if f.contains("wrap") {
                                    past_page_no = page_no;
                                    page_no = 0;
                                } else {
                                    data.ebook.push_back(Page::new());
                                }

                                let res = archive.get_entry_as_str(f.clone());

                                if res.is_ok() {
                                    let init = res.as_ref().unwrap().find("<?xml");

                                    if res.as_ref().unwrap()[init.unwrap()..].contains("class=\"x-ebookmaker-cover\"") {
                                        chapter_title = "COVER".to_string();
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("START OF THIS PROJECT GUTENBERG EBOOK") {
                                        chapter_title = "START OF THIS PROJECT GUTENBERG EBOOK".to_string();
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("END OF THIS PROJECT GUTENBERG EBOOK") {
                                        chapter_title = "END OF THIS PROJECT GUTENBERG EBOOK".to_string();
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("CONTENTS") {
                                        chapter_title = "CONTENTS".to_string();
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("pgepubid00000") {
                                        let inizio = res.as_ref().unwrap().find("<title>").unwrap();
                                        let fine = res.as_ref().unwrap().find("</title>").unwrap();
                                        chapter_title = strip_tags(&res.as_ref().unwrap()[inizio..fine].replace("\n", "").trim_start().trim_end());
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("PREFACE") {
                                        chapter_title = "PREFACE".to_string();
                                    } else if res.as_ref().unwrap()[init.unwrap()..].contains("ILLUSTRATIONS") {
                                        chapter_title = "ILLUSTRATIONS".to_string();
                                    } else if res.as_ref().unwrap().find("<div class=\"chapter\"").is_none() {
                                        chapter_title = "POSTFACE".to_string();
                                    } else {
                                        chapter_title = strip_tags(&res.as_ref().unwrap()[res.as_ref().unwrap().find("<div class=\"chapter\"").unwrap()..res.as_ref().unwrap().find("</div>").unwrap()])
                                            .replace("\n", " ").trim_start().trim_end().to_string();
                                    }

                                    if chapter_title.eq("COVER") {
                                        data.chapters.push_front(Chapter::from(chapter_title, page_no));
                                    } else {
                                        data.chapters.push_back(Chapter::from(chapter_title, page_no));
                                    }


                                    let page_occ = res.as_ref().unwrap()[init.unwrap()..].matches("<span class=\"x-ebookmaker-pageno\"").count();

                                    if page_occ > 0 {
                                        let mut pos_pageno = res.as_ref().unwrap()[init.unwrap()..].find("<span class=\"x-ebookmaker-pageno\"").unwrap();
                                        pos_pageno += res.as_ref().unwrap()[init.unwrap() + pos_pageno..].find("</span>").unwrap() + 7;
                                        let mut text = res.as_ref().unwrap()[init.unwrap()..pos_pageno].to_string().clone();
                                        let mut img_occ = text.matches("<img").count();
                                        let mut pos = text.find("<img");
                                        if img_occ > 0 {
                                            if pos.is_some() {
                                                let mut displacement: usize = 0;
                                                for _ in 0..img_occ {
                                                    let mut s1 = String::from("OEBPS/");
                                                    let app = text[pos.unwrap() + 3 + displacement..].find("src=");
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

                                                    let r;

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
                                                    if page_not_ended {
                                                        data.ebook[page_no - 1].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                    } else {
                                                        data.ebook[page_no].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                    }
                                                    let resapp = text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                    displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                    pos = resapp;
                                                }
                                            }
                                        }

                                        if !page_not_ended {
                                            data.ebook[page_no].text.push_str(text.as_str());
                                            data.ebook.push_back(Page::new());
                                            page_no += 1;
                                        } else {
                                            data.ebook[page_no - 1].text.push_str(text.as_str());
                                            page_not_ended = false;
                                        }

                                        pos_pageno += 1;

                                        for _ in 1..page_occ {
                                            let mut next_page = res.as_ref().unwrap()[init.unwrap() + pos_pageno..].find("<span class=\"x-ebookmaker-pageno\"").unwrap();
                                            next_page += res.as_ref().unwrap()[init.unwrap() + pos_pageno + next_page..].find("</span>").unwrap() + 6;
                                            let text = res.as_ref().unwrap()[init.unwrap() + pos_pageno..init.unwrap() + pos_pageno + next_page + 1].to_string().clone();
                                            let img_occ = text.matches("<img").count();
                                            let mut pos = text.find("<img");
                                            if img_occ > 0 {
                                                if pos.is_some() {
                                                    let mut displacement: usize = 0;
                                                    for _ in 0..img_occ {
                                                        let mut s1 = String::from("OEBPS/");
                                                        let app = text[pos.unwrap() + 3 + displacement..].find("src=");
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

                                                        let r;

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
                                                        if page_not_ended {
                                                            data.ebook[page_no - 1].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                        } else {
                                                            data.ebook[page_no].images.push_back(ImageOfPage::from(Vector::from(result), width, height));
                                                        }
                                                        let resapp = text[pos.unwrap() + 3 + app.unwrap() + 5 + displacement..].find("<img");
                                                        displacement = pos.unwrap() + 3 + app.unwrap() + 5 + displacement;
                                                        pos = resapp;
                                                    }
                                                }
                                            }

                                            data.ebook[page_no].text.push_str(text.as_str());
                                            data.ebook.push_back(Page::new());
                                            page_no += 1;
                                            pos_pageno += next_page + 1;
                                        }

                                        page_not_ended = true;
                                        text = res.as_ref().unwrap()[init.unwrap() + pos_pageno..res.as_ref().unwrap().len()].to_string().clone();
                                        img_occ = text.matches("<img").count();
                                        pos = text.find("<img");

                                        if img_occ > 0 {
                                            if pos.is_some() {
                                                let mut displacement: usize = 0;
                                                for _ in 0..img_occ {
                                                    let mut s1 = String::from("OEBPS/");
                                                    let app = text[pos.unwrap() + 3 + displacement..].find("src=");
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

                                                    let r;

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

                                        data.ebook.push_back(Page::new());
                                        page_no += 1;
                                    } else {
                                        page_not_ended = false;
                                        let img_occ = res.as_ref().unwrap()[init.unwrap()..].matches("<img").count();
                                        let mut pos = res.as_ref().unwrap()[init.unwrap()..].find("<img");
                                        if img_occ > 0 {
                                            if pos.is_some() {
                                                let mut displacement: usize = 0;
                                                for _ in 0..img_occ {
                                                    let mut s1 = String::from("OEBPS/");
                                                    let app = res.as_ref().unwrap()[pos.unwrap() + 3 + displacement..].find("src=");
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

                                                    let r;
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
                                        data.ebook[page_no].text.push_str(res.as_ref().unwrap()[init.unwrap()..res.as_ref().unwrap().len()].to_string().clone().as_str());

                                        page_no += 1;
                                    }
                                }
                                if f.contains("wrap") {
                                    page_no = past_page_no;
                                }
                            }
                        }

                        while data.ebook.last().unwrap().text.len() == 0 {
                            data.ebook.pop_back();
                        }
                    }
                    Err(error) => {
                        println!("Error while opening archive: {}", error);
                    }
                }
                return Handled::Yes;
            }
        }
        Handled::No
    }
}

