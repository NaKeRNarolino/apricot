use std::time::Duration;
use super::navigation::Navigator;

#[derive(Clone)]
pub struct Context {
    navigator: Navigator,
    exists_for: Duration
}

impl Context {
    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn create(navigator: Navigator) -> Self {
        Self { navigator, exists_for: Duration::default() }
    }
}
