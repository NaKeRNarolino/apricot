// use crate::core::widget::WidgetArray;
// 
// use super::context::Context;
// 
// pub type ScreenArray = Vec<Screen>;
// pub type ScreensProvider = dyn Fn(Context) -> ScreenArray;
// 
// #[derive(Clone)]
// pub struct Screen {
//     pub path: String,
//     pub widgets: WidgetArray,
// }
// 
// impl Screen {
//     pub fn create(path: impl Into<String>, widgets: WidgetArray) -> Self {
//         Self {
//             path: path.into(),
//             widgets,
//         }
//     }
// }
