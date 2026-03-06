use tokio::sync::Mutex;

use crate::{api::{NexApiClient, types::{locations::{InstitutionLocations, LocationsQuery}, nex_api::NexApiResponse}}, services::processors::traits::Processor};

pub struct Controller {
    pub subdomain: Mutex<Option<String>>,

    pub processor: Mutex<Option<Box<dyn Processor + Send + Sync>>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            subdomain: Mutex::new(None),
            processor: Mutex::new(None),
        }
    }

    pub async fn get_locations(&self, client: &NexApiClient, inactive: bool) -> Result<NexApiResponse<Vec<InstitutionLocations>>, String> {
        let guard = self.subdomain.lock().await;

        let subdomain = guard
            .as_ref()
            .ok_or("No subdomain stored in controller")?;

        let response = client.get_locations(LocationsQuery {
            subdomain: subdomain.clone(),
            inactive,
        }).await.map_err(|e| e.to_string())?;

        Ok(response)
    }
}
