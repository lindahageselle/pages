use crate::router::Route;
use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(back_button())
        .item(link("Home", Route::Root))
        .item(link("Crab", Route::Crab))
        .item(link("Fox", Route::Fox))
        .item(link("Cat", Route::Cat))
        .item(link("Aloy", Route::Aloy))
        .item(link("Pond", Route::Pond))
}

fn back_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(
            hovered_signal.map_bool(|| hsluv!(18, 100, 58, 100), || hsluv!(18, 100, 48, 100)),
        ))
        .s(Padding::new().x(7).y(4))
        .s(Font::new().color(GRAY_4))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("< Back")
        .on_press(routing::back)
}

fn link(label: &str, route: Route) -> impl Element {
    Link::new()
        .s(Font::new()
            .color(hsluv!(18, 100, 48, 100))
            .line(FontLine::new().underline()))
        .label(label)
        .to(route)
}
