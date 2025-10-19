use anyhow::anyhow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsRequestAllUuidTimestamps;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsRequestVcardsByUuid {
    pub uids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsResponseUidsTimestamps {
    pub uids: Vec<String>,

    #[serde(flatten)]
    pub extras: HashMap<String, u64>,
}

impl ContactsResponseUidsTimestamps {
    /// Convierte a HashMap<String, i64> sólo con las keys listadas en `uids`
    pub fn into_uid_timestamps(self) -> Result<HashMap<String, u64>, anyhow::Error> {
        let mut map = HashMap::new();
        for uid in &self.uids {
            match self.extras.get(uid) {
                Some(v) => {
                    map.insert(uid.clone(), *v);
                }
                None => return Err(anyhow!("timestamp faltante para uid '{}'", uid)),
            }
        }
        Ok(map)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsResponseVcards {
    pub uids: Vec<String>,

    #[serde(flatten)]
    pub extras: HashMap<String, String>,
}

impl ContactsResponseVcards {
    /// Convierte a HashMap<String, String> sólo con las keys listadas en `uids`
    pub fn into_uid_vcard(self) -> Result<HashMap<String, String>, anyhow::Error> {
        let mut map = HashMap::new();
        for uid in &self.uids {
            if let Some(v) = self.extras.get(uid) {
                map.insert(uid.clone(), v.clone());
            } else {
                return Err(anyhow!("Falta VCard para uid '{}'", uid));
            }
        }
        Ok(map)
    }
}
