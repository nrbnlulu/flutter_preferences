use std::sync::Mutex;

use lazy_static::lazy_static;
use preferences::{AppInfo, Preferences, PreferencesMap};

lazy_static! {
    pub(crate) static ref APP_PREFS: Mutex<Option<AppPrefs>> = Mutex::new(None);
}

pub(crate) struct AppPrefs {
    pub app_info: AppInfo,
    pub cached_prefs: std::sync::Mutex<PreferencesMap<String>>,
}
impl AppPrefs {
    pub fn new(app_info: AppInfo) -> Self {
        let cached_prefs = match PreferencesMap::<String>::load(&app_info, "default".to_string()) {
            Ok(prefs) => prefs,
            Err(_) => PreferencesMap::new(),
        };
        AppPrefs {
            app_info,
            cached_prefs: std::sync::Mutex::new(cached_prefs),
        }
    }
    pub fn get_string(&self, key: String) -> Option<String> {
        let prefs = self.cached_prefs.lock().unwrap();
        prefs.get(&key).cloned()
    }
    pub fn set_string(&self, key: String, value: String) {
        let mut prefs = self.cached_prefs.lock().unwrap();
        prefs.insert(key.clone(), value);
        prefs
            .save(&self.app_info, key)
            .expect("Failed to save preferences");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_prefs_new() {
        let app_info = AppInfo {
            name: "test_app",
            author: "test_org",
        };
        let app_prefs = AppPrefs::new(app_info.clone());
        assert_eq!(app_prefs.app_info, app_info);
    }

    #[test]
    fn test_get_string() {
        let app_info = AppInfo {
            name: "test_app",
            author: "test_org",
        };
        let app_prefs = AppPrefs::new(app_info);
        app_prefs.set_string("key1".to_string(), "value1".to_string());
        assert_eq!(
            app_prefs.get_string("key1".to_string()),
            Some("value1".to_string())
        );
        assert_eq!(app_prefs.get_string("key2".to_string()), None);
    }

    #[test]
    fn test_set_string() {
        let app_info = AppInfo {
            name: "test_app",
            author: "test_org",
        };
        let app_prefs = AppPrefs::new(app_info);
        app_prefs.set_string("key1".to_string(), "value1".to_string());
        assert_eq!(
            app_prefs.get_string("key1".to_string()),
            Some("value1".to_string())
        );
        app_prefs.set_string("key1".to_string(), "value2".to_string());
        assert_eq!(
            app_prefs.get_string("key1".to_string()),
            Some("value2".to_string())
        );
    }
}
