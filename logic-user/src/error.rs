#[derive(Debug, thiserror::Error)]
pub enum PlacementError {
    #[error("No location left possible when plandoing!")]
    PlandoNoLocation,
    #[error("No location left possible when doing fill!")]
    NoLocation,
}
