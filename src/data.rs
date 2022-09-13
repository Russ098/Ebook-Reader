#![allow(non_snake_case)]
use druid::{im::Vector, Data, Lens, EventCtx, Env, ArcStr, KeyOrValue, FontFamily};
use std::fs::File;
use std::io::{BufReader, Error};
use druid::widget::TextBox;
use druid::text::{RichText, Attribute};

const SIZE_FONT: f64 = 40.0;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    font_size: String,
    rich_text: RichText
}

impl AppState {
    pub fn new() -> Self {
        Self {
            font_size: SIZE_FONT.to_string(),
            rich_text: RichText::new(ArcStr::from("Nel mezzo del cammin di nostra vita
mi ritrovai per una selva oscura
ché la diritta via era smarrita. 3
Ahi quanto a dir qual era è cosa dura
esta selva selvaggia e aspra e forte
che nel pensier rinova la paura! 6
Tant’è amara che poco è più morte;
ma per trattar del ben ch’i’ vi trovai,
dirò de l’altre cose ch’i’ v’ho scorte. 9
Io non so ben ridir com’i’ v’intrai,
tant’era pien di sonno a quel punto
che la verace via abbandonai.Nel mezzo del cammin di nostra vita
mi ritrovai per una selva oscura
ché la diritta via era smarrita. 3
Ahi quanto a dir qual era è cosa dura
esta selva selvaggia e aspra e forte
che nel pensier rinova la paura! 6
Tant’è amara che poco è più morte;
ma per trattar del ben ch’i’ vi trovai,
dirò de l’altre cose ch’i’ v’ho scorte. 9
Io non so ben ridir com’i’ v’intrai,
tant’era pien di sonno a quel punto
che la verace via abbandonai.")).with_attribute(..,Attribute::FontSize(KeyOrValue::Concrete(SIZE_FONT)))
                .with_attribute(..,Attribute::FontFamily(FontFamily::SANS_SERIF))
        }
    }
}


