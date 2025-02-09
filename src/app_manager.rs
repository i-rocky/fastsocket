use crate::app::App;
use std::sync::Arc;

pub trait AppManager: Send + Sync {
    fn find(&self, id: &str) -> Option<Arc<App>>;
    fn find_by_id(&self, id: &str) -> Option<Arc<App>> {
        self.find(id)
    }
    fn find_by_key(&self, key: &str) -> Option<Arc<App>>;
    fn find_by_secret(&self, secret: &str) -> Option<Arc<App>>;
    fn add(&mut self, app: Arc<App>);
    fn update(&mut self, app: Arc<App>);
    fn remove(&mut self, id: &str) -> bool;
}
