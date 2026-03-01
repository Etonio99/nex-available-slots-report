pub trait Advancement {
    fn advance(&self) -> Result<(), Box<dyn std::error::Error>>;
}