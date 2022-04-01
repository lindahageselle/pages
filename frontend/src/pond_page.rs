use zoon::{named_color::*, *};
use zoon::{Background, Column, Element, RawEl, RawHtmlEl};

pub fn page() -> impl Element {
    Column::new()
        .s(Padding::top(Default::default(), 40))
        .item(amelia())
        .s(Align::new().center_x())
}

fn amelia() -> impl Element {
    RawHtmlEl::new("div")
    .style("height", "500px")
    .style("width", "900px")
    .style("background-color", "#ffffff")
    .style("display", "flex")
    .style("flex-direction", "row")
    .style("justify-content", "center")
    .style("align-items", "center")
    .inner_markup("<img src='http://4.bp.blogspot.com/-FIxRWVF3jds/VL2Eq2l4w-I/AAAAAAAACno/wdR-BvOWjPo/s1600/amy%2Bpond%2B1.jpg' 
    alt='hi' width='400' style='border: 5px solid black'/>")
}
