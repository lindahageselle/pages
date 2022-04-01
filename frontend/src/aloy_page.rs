use zoon::{*, named_color::*};

pub fn page() -> impl Element {
    Column::new()
    .s(Padding::top(Default::default(), 40))
    .item(picture())
        .s(Align::new().center_x())
        .s(Spacing::new(50))
}

fn picture() -> impl Element {
    Image::new()
        .url("https://asset.vg247.com/horizon_forbidden_west_artwork_crop.jpg/BROK/thumbnail/1600x900/quality/100/horizon_forbidden_west_artwork_crop.jpg")
        .description("A fox")
        .s(Width::new(900))
}