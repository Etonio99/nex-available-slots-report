use crate::api::NexApiClient;

pub trait Advancement {
    fn advance(&mut self, client: &NexApiClient) -> Result<(), Box<dyn std::error::Error>>;
}