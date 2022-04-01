use zoon::{*, named_color::*, web_sys::Window};

pub fn page() -> impl Element {
    Column::new()
    .s(Padding::top(Default::default(), 40))
    .item(picture())
        .s(Align::new().center_x())
        .s(Spacing::new(50))
    // .item(div_test())
}

fn picture() -> impl Element {
    Image::new()
        .url("https://cdn2.psychologytoday.com/assets/styles/manual_crop_1_91_1_1528x800/public/field_blog_entry_teaser_image/2020-09/teamk_pixabay.jpg?itok=a3QpJVQ6")
        .description("A cat")
        .s(Width::new(600))

}

// fn div_test() -> impl Element {
//     RawHtmlEl::new("div")
//     .style("height", "400px")
//     .style("width", "500px")
//     .style("background-color", "#00ff88")
//     .inner_markup("<img src='https://asset.vg247.com/horizon_forbidden_west_artwork_crop.jpg/BROK/thumbnail/1600x900/quality/100/horizon_forbidden_west_artwork_crop.jpg' alt='hi' width='400' style='align:center;cursor:grab'/>")
// }

