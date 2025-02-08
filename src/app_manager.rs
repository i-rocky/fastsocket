use crate::app::App;

pub trait AppManager {
    fn find(&self, id: &str) -> Option<&App>;
    fn find_by_id(&self, id: &str) -> Option<&App> {
        self.find(id)
    }
    fn find_by_key(&self, key: &str) -> Option<&App>;
    fn find_by_secret(&self, secret: &str) -> Option<&App>;
    fn add(&mut self, app: App);
    fn update(&mut self, app: App);
    fn remove(&mut self, id: &str) -> bool;
}
