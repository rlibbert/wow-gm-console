use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use directories::ProjectDirs;
use uuid::Uuid;

use super::keychain;
use super::model::ServerProfile;

pub struct ProfileStore {
    path: PathBuf,
    profiles: Mutex<Vec<ServerProfile>>,
}

fn config_file_path() -> Result<PathBuf, String> {
    let dirs = ProjectDirs::from("com", "rl", "wow-gm-console")
        .ok_or_else(|| "could not determine config directory for this OS".to_string())?;
    let dir = dirs.config_dir();
    fs::create_dir_all(dir).map_err(|e| format!("failed to create config dir: {e}"))?;
    Ok(dir.join("profiles.json"))
}

impl ProfileStore {
    pub fn load() -> Result<Self, String> {
        let path = config_file_path()?;
        let profiles = if path.exists() {
            let raw = fs::read_to_string(&path)
                .map_err(|e| format!("failed to read profiles file: {e}"))?;
            serde_json::from_str(&raw).map_err(|e| format!("failed to parse profiles file: {e}"))?
        } else {
            Vec::new()
        };
        Ok(Self {
            path,
            profiles: Mutex::new(profiles),
        })
    }

    fn persist(&self, profiles: &[ServerProfile]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(profiles)
            .map_err(|e| format!("failed to serialize profiles: {e}"))?;
        fs::write(&self.path, json).map_err(|e| format!("failed to write profiles file: {e}"))
    }

    pub fn list(&self) -> Vec<ServerProfile> {
        self.profiles.lock().unwrap().clone()
    }

    pub fn add(
        &self,
        name: String,
        host: String,
        soap_port: u16,
        username: String,
        password: String,
    ) -> Result<ServerProfile, String> {
        let profile = ServerProfile {
            id: Uuid::new_v4(),
            name,
            host,
            soap_port,
            username,
        };

        keychain::set_password(profile.id, &password)?;

        let mut profiles = self.profiles.lock().unwrap();
        profiles.push(profile.clone());
        self.persist(&profiles)?;
        Ok(profile)
    }

    pub fn update(
        &self,
        id: Uuid,
        name: String,
        host: String,
        soap_port: u16,
        username: String,
        password: Option<String>,
    ) -> Result<ServerProfile, String> {
        let mut profiles = self.profiles.lock().unwrap();
        let entry = profiles
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| "profile not found".to_string())?;

        entry.name = name;
        entry.host = host;
        entry.soap_port = soap_port;
        entry.username = username;

        if let Some(password) = password {
            keychain::set_password(id, &password)?;
        }

        let updated = entry.clone();
        self.persist(&profiles)?;
        Ok(updated)
    }

    pub fn remove(&self, id: Uuid) -> Result<(), String> {
        let mut profiles = self.profiles.lock().unwrap();
        profiles.retain(|p| p.id != id);
        self.persist(&profiles)?;
        keychain::delete_password(id)?;
        Ok(())
    }

    pub fn get(&self, id: Uuid) -> Result<ServerProfile, String> {
        self.profiles
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or_else(|| "profile not found".to_string())
    }
}
