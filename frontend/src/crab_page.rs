use zoon::{*, named_color::*};

pub fn page() -> impl Element {
    Column::new()
    .s(Padding::top(Default::default(), 40))
        .item(picture_row())
        .s(Align::new().center_x())
        .s(Spacing::new(50))
        .item(crab_text())
}

fn picture_row() -> impl Element {
    Row::new()
    .item(picture())
    .item(second_picture())
}

fn second_picture() -> impl Element {
    Image::new()
    .url("https://rustacean.net/assets/rustacean-flat-happy.png")
    .description("A crab")
    .s(Background::new().color(hsluv!(0, 0, 0, 100)))
    .s(Width::new(600))
    
}

fn picture() -> impl Element {
    Image::new()
        .url("https://rustacean.net/assets/rustacean-flat-happy.png")
        .description("A crab")
        .s(Background::new().color(BLUE_4))
        .s(Width::new(600))
        // .s(Padding::left(Default::default(), 300))
}

fn crab_text() -> impl Element {
    Paragraph::new()
    .s(Font::new().size(24).color(hsluv!(0, 0, 0, 100)))
    .s(Background::new().color(hsluv!(18,100,48,100)))
    .s(Width::new(400))
    .s(Align::new().center_x())
    .content("We are crabs! This is weird...")
}
