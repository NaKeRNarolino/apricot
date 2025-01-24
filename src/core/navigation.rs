use core::panic;
use std::collections::HashMap;

// use super::screen::{Screen, ScreenArray};

// #[derive(Clone)]
// pub struct NavigatorConfig {
//     pub home_screen: String,
//     pub screens: ScreenArray,
//     pub path_screen: HashMap<String, Screen>,
// }

// impl NavigatorConfig {
//     pub fn create_from_screens(home: String, screens: ScreenArray) -> Self {
//         let mut hash: HashMap<String, Screen> = HashMap::new();

//         for screen in screens.clone() {
//             hash.insert(screen.clone().path, screen.clone());
//         }

//         Self {
//             screens,
//             home_screen: home,
//             path_screen: hash,
//         }
//     }
// }

#[derive(Clone)]
pub struct Navigator {
    // config: NavigatorConfig,
    current: String,
    pages: Vec<String>,
}

impl Navigator {
    pub fn based_on<P>(home: P, pages: Vec<P>) -> Self
    where
        P: Into<String>,
        P: Clone,
        P: ToString,
    {
        Self {
            current: home.into(),
            pages: pages.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn navigate_to<P>(&mut self, path: P)
    where
        P: Into<String>,
        P: Clone,
    {
        if !self.pages.contains(&path.clone().into()) {
            panic!("The path {} doesn't exist on the App", path.clone().into())
        }
        self.current = path.into();
    }

    pub fn current_screen_path(&self) -> String {
        self.current.clone()
    }
}
