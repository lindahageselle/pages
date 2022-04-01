use zoon::dominator::traits::AsStr;
use zoon::serde::de::Unexpected::Option;
use zoon::{eprintln, *};

#[static_ref]
fn fox_text() -> &'static Mutable<&'static str> {
    Mutable::new("Foxes are cool!")
}

fn set_fox_text() {
    // fox_text().set("Foxy!");   //this works to change text once

    fox_text().update(|fox_text| {
        if fox_text.eq("Foxes are cool!") {
            "Foxy!"
        } else {
            "Foxes are cool!"
        }
    });
}

pub fn page() -> impl Element {
    Column::new()
        .item(fox_button())
        .item(Text::with_signal(fox_text().signal()))
}

fn fox_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(20))
        .s(Background::new().color_signal(
            hovered_signal.map_bool(|| hsluv!(18, 100, 48, 100), || hsluv!(18, 100, 58, 100)),
        ))
        .s(Font::new().color(hsluv!(0, 100, 0, 100)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .on_press(set_fox_text)
        .label("Click to change the text!")
}
