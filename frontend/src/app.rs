use crate::{
    aloy_page, cat_page, crab_page, fox_page,
    header::header,
    pond_page,
    router::{previous_route, router, Route},
};
// use shared::{DownMsg, UpMsg, User};
use zoon::*;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Home,
    Crab,
    Fox,
    Cat,
    Aloy,
    Pond,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
pub fn logged_user() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//    Helpers
// ------ ------

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20))
        .s(Spacing::new(20))
        .item(header())
        .item(page())
}

// fn front_page() -> impl Element {
//
// }

fn page() -> impl Element {
    // connect();
    El::new().child_signal(page_id().signal().map(|page_id| {
        match page_id {
            PageId::Home => El::new()
                .s(Padding::top(Default::default(), 250))
                .child("Welcome Home!")
                .s(Font::new().size(40).color(hsluv!(18, 100, 48, 100)))
                .s(Align::new().center_x())
                .s(Align::new().center_y())
                .into_raw_element(),

            PageId::Crab => crab_page::page().into_raw_element(),
            PageId::Fox => fox_page::page().into_raw_element(),
            PageId::Cat => cat_page::page().into_raw_element(),
            PageId::Unknown => El::new().child("404").into_raw_element(),
            PageId::Aloy => aloy_page::page().into_raw_element(),
            PageId::Pond => pond_page::page().into_raw_element(),
        }
    }))
}

// #[static_ref]
// fn users() -> &'static MutableVec<User> {
//     MutableVec::new()
// }

// #[static_ref]
// pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
//     Connection::new(|DownMsg::MessageReceived(message), _| {
//         users().lock_mut().push_cloned(message);
//     })
// }
