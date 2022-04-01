use crate::{
    app::{self, PageId}
};
use std::collections::VecDeque;
use zoon::{println, *};

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| {
        println!("{}", routing::url());

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return app::set_page_id(PageId::Unknown);
            }
        };

        match route {
            Route::Crab => {
                app::set_page_id(PageId::Crab);
            }
            Route::Fox => {
                app::set_page_id(PageId::Fox);
            }

            Route::Cat => {
                app::set_page_id(PageId::Cat);
            }

            Route::Aloy => {
                app::set_page_id(PageId::Aloy);
            }

            Route::Pond => {
                app::set_page_id(PageId::Pond)
            }

            Route::Root => {
                app::set_page_id(PageId::Home);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {

    #[route("crab")]
    Crab,

    #[route("fox")]
    Fox,

    #[route("cat")]
    Cat,

    #[route("aloy")]
    Aloy,

    #[route("pond")]
    Pond,

    #[route()]
    Root,
}
