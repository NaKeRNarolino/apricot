// pub type WidgetArray = Vec<Arc<dyn Widget>>;


// impl Widget for Row {
//     fn make_ui(&self, ui: &mut eframe::egui::Ui) {
//         let mut children = self.children.clone();
//         if self.reversed {
//             children.reverse();
//         }
//         ui.allocate_ui_with_layout(
//             vec2(
//                 0.0,
//                 match self.cross_axis_occupy_policy {
//                     OccupyPolicy::Minimal => 0.0,
//                     OccupyPolicy::Full => ui.available_size().y,
//                     OccupyPolicy::Custom(v) => v,
//                 },
//             ),
//             Layout::left_to_right(match self.cross_axis_alignment {
//                 CrossAxisAlignment::Start => Align::TOP,
//                 CrossAxisAlignment::End => Align::BOTTOM,
//                 CrossAxisAlignment::Center => Align::Center,
//             }),
//             |ui| {
//                 for child in children {
//                     child.make_ui(ui);
//                 }
//             },
//         );
//     }
// }
// 
// impl Row {
//     pub fn create(children: WidgetArray) -> Self {
//         Self {
//             children,
//             cross_axis_alignment: CrossAxisAlignment::Start,
//             reversed: false,
//             cross_axis_occupy_policy: OccupyPolicy::Minimal,
//         }
//     }
// 
//     pub fn with_cross_axis_alignment(&mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
//         self.cross_axis_alignment = cross_axis_alignment;
// 
//         self.clone()
//     }
// 
//     pub fn with_reversed(&mut self, reversed: bool) -> Self {
//         self.reversed = reversed;
// 
//         self.clone()
//     }
// 
//     pub fn with_cross_axis_occupy_policy(&mut self, policy: OccupyPolicy) -> Self {
//         self.cross_axis_occupy_policy = policy;
// 
//         self.clone()
//     }
// }
// 
// #[derive(Clone)]
// pub struct ScrollableArea {
//     child: Arc<dyn Widget>,
//     orientation: Orientation,
// }
// 
// impl Widget for ScrollableArea {
//     fn make_ui(&self, ui: &mut eframe::egui::Ui) {
//         match self.orientation {
//             Orientation::Vertical => ScrollArea::vertical().show(ui, |ui| {
//                 self.child.make_ui(ui);
//             }),
//             Orientation::Horizontal => ScrollArea::horizontal().show(ui, |ui| {
//                 self.child.make_ui(ui);
//             }),
//         };
//     }
// }
// 
// impl ScrollableArea {
//     pub fn create(child: Arc<dyn Widget>) -> Self {
//         Self {
//             child,
//             orientation: Orientation::Vertical,
//         }
//     }
// 
//     pub fn with_orientation(&mut self, orientation: Orientation) -> Self {
//         self.orientation = orientation;
// 
//         self.clone()
//     }
// }
// 
// #[derive(Clone)]
// pub struct Column {
//     children: WidgetArray,
//     cross_axis_alignment: CrossAxisAlignment,
//     reversed: bool,
// }
// 
// impl<'a> Widget for Column {
//     fn make_ui(&self, ui: &mut eframe::egui::Ui) {
//         let mut children = self.children.clone();
//         if self.reversed {
//             children.reverse();
//         }
//         ui.allocate_ui_with_layout(
//             vec2(f32::INFINITY, 0.0),
//             Layout::top_down(match self.cross_axis_alignment {
//                 CrossAxisAlignment::Start => Align::TOP,
//                 CrossAxisAlignment::End => Align::BOTTOM,
//                 CrossAxisAlignment::Center => Align::Center,
//             }),
//             |ui| {
//                 for child in children {
//                     child.make_ui(ui);
//                 }
//             },
//         );
//     }
// }
// 
// impl Column {
//     pub fn create(children: WidgetArray) -> Self {
//         Self {
//             children,
//             cross_axis_alignment: CrossAxisAlignment::Start,
//             reversed: false,
//         }
//     }
// 
//     pub fn with_cross_axis_alignment(&mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
//         self.cross_axis_alignment = cross_axis_alignment;
// 
//         self.clone()
//     }
// 
//     pub fn with_reversed(&mut self, reversed: bool) -> Self {
//         self.reversed = reversed;
// 
//         self.clone()
//     }
// }
// 
// #[derive(Clone)]
// pub struct FilledButton<'a> {
//     text: String,
//     on_clicked: &'a dyn Fn(),
// }
// 
// impl<'a> Widget for FilledButton<'a> {
//     fn make_ui(&self, ui: &mut eframe::egui::Ui) {
//         let button = ui.button(self.text.clone());
// 
//         if button.clicked() {
//             (self.on_clicked)()
//         }
//     }
// }
// 
// impl<'a> ApplyProperties for FilledButton<'a> {}
// 
// impl<'a> FilledButton<'a> {
//     pub fn create(text: impl Into<String>, on_clicked: &'a impl Fn()) -> Self {
//         Self {
//             text: text.into(),
//             on_clicked,
//         }
//     }
// }
