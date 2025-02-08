use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::app::App;
use crate::app_manager::AppManager;


#[derive(Default, Debug)]
struct Indices {
    by_key: HashMap<String, String>,
    by_secret: HashMap<String, String>,
}


#[derive(Debug)]
pub struct JsonAppManager {
    path: PathBuf,
    apps: HashMap<String, App>,
    indices: Indices,
    dirty: bool,
}

impl JsonAppManager {
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        let (apps, indices) = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let app_vec: Vec<App> = serde_json::from_str(&content)?;
            let mut indices = Indices::default();
            let apps: HashMap<_, _> = app_vec.into_iter()
                .map(|app| {
                    let id = app.get_id().to_string();
                    indices.by_key.insert(app.get_key().to_string(), id.clone());
                    let secret = app.get_secret().to_string();
                    indices.by_secret.insert(secret, id.clone());
                    (id, app)
                })
                .collect();
            (apps, indices)
        } else {
            (HashMap::with_capacity(16), Indices::default())
        };

        Ok(JsonAppManager {
            path,
            apps,
            indices,
            dirty: false,
        })
    }

    #[inline]
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.dirty {
            return Ok(());
        }

        let apps: Vec<_> = self.apps.values().collect();
        let content = serde_json::to_string_pretty(&apps)?;
        std::fs::write(&self.path, content)?;
        self.dirty = false;
        Ok(())
    }
}

impl AppManager for JsonAppManager {
    #[inline]
    fn find(&self, id: &str) -> Option<&App> {
        self.apps.get(id)
    }

    #[inline]
    fn find_by_key(&self, key: &str) -> Option<&App> {
        self.indices.by_key.get(key).and_then(|id| self.apps.get(id))
    }

    #[inline]
    fn find_by_secret(&self, secret: &str) -> Option<&App> {
        self.indices.by_secret.get(secret).and_then(|id| self.apps.get(id))
    }

    #[inline]
    fn add(&mut self, app: App) {
        let id = app.get_id().to_string();
        let key = app.get_key().to_string();
        let secret = app.get_secret().to_string();

        self.indices.by_key.insert(key, id.clone());
        self.indices.by_secret.insert(secret, id.clone());

        self.apps.insert(id, app);
        self.dirty = true;
    }

    #[inline]
    fn update(&mut self, app: App) {
        let id = app.get_id().to_string();
        if let Some(old_app) = self.apps.get(&id) {
            self.indices.by_key.remove(old_app.get_key());
            self.indices.by_secret.remove(old_app.get_secret());
        }

        let key = app.get_key().to_string();
        let secret = app.get_secret().to_string();

        self.indices.by_key.insert(key, id.clone());
        self.indices.by_secret.insert(secret, id.clone());

        self.apps.insert(id, app);
        self.dirty = true;
    }

    #[inline]
    fn remove(&mut self, id: &str) -> bool {
        if let Some(app) = self.apps.remove(id) {
            self.indices.by_key.remove(app.get_key());
            self.indices.by_secret.remove(app.get_secret());
            self.dirty = true;
            true
        } else {
            false
        }
    }
}

impl Drop for JsonAppManager {
    fn drop(&mut self) {
        if self.dirty {
            let _ = self.save();
        }
    }
}
