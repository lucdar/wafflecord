use anyhow::Result;
use poise::serenity_prelude::{ChannelId, RoleId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone)]
pub struct SubscriptionStore {
    db: sled::Db,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Subscriber {
    pub channel_id: ChannelId,
    pub role_id: Option<RoleId>,
}

impl SubscriptionStore {
    pub fn try_load(path: PathBuf) -> Result<Self> {
        match sled::open(path) {
            Ok(db) => {
                println!("Loaded {} subscribers.", db.len());
                Ok(SubscriptionStore { db })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Returns an iterator over all of the current subscribers
    pub fn subscribers_iter(&self) -> impl Iterator<Item = Subscriber> {
        self.db.iter().filter_map(|item| match item {
            Ok((_id, value)) => bincode::deserialize::<Subscriber>(&value)
                .inspect_err(|e| {
                    eprintln!("Error parsing Subscriber {e}");
                })
                .ok(),
            Err(e) => {
                eprintln!("Error reading from db {e}");
                None
            }
        })
    }

    pub fn add_subscriber(&self, subscriber: Subscriber) -> Result<()> {
        let id = SubscriptionStore::channel_id_to_bytes(&subscriber.channel_id);
        self.db.insert(id, bincode::serialize(&subscriber)?)?;
        Ok(())
    }

    pub fn remove_subscriber(
        &self,
        channel_id: ChannelId,
    ) -> Result<Option<Subscriber>> {
        let id = SubscriptionStore::channel_id_to_bytes(&channel_id);
        Ok(self
            .db
            .remove(id)?
            .map(|bytes| bincode::deserialize(&bytes))
            .transpose()?)
    }

    fn channel_id_to_bytes(channel_id: &ChannelId) -> [u8; 8] {
        channel_id.get().to_be_bytes()
    }
}
