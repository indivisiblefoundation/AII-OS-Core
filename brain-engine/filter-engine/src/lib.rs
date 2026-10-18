#[derive(Debug, Clone)]
pub struct RawInput {
    pub subject: String,
    pub payload: Vec<u8>,
    pub ts: u64,
}

#[derive(Debug, Clone)]
pub struct FilteredInput {
    pub subject: String,
    pub payload: Vec<u8>,
}

pub trait Filter: Send + Sync {
    fn name(&self) -> &str;
    fn filter(&self, input: RawInput) -> Option<FilteredInput>;
}

// Example: drop duplicate in 5 sec window (logic to implement with Qdrant)
pub struct DedupeFilter;
pub struct RateLimitFilter;
pub struct PiiFilter;
