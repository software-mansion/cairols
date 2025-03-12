use std::hash::{DefaultHasher, Hash, Hasher};

use serde::Serialize;

use super::AnalysisDatabase;

#[derive(Clone, Default, Debug, Serialize, Hash)]
pub enum EventKind {
    DidValidateMemoizedValue,
    WillBlockOn,
    WillExecute,
    #[default]
    WillCheckCancellation,
}

#[derive(Clone, Default, Debug, Serialize, Hash)]
pub enum MaybeMissing<T> {
    #[serde(rename = "null")]
    #[default]
    Missing,
    Some(T),
}

impl<T: Sized> MaybeMissing<T> {
    pub fn is_some_and(&self, fun: impl FnOnce(&T) -> bool) -> bool {
        match self {
            MaybeMissing::Missing => false,
            MaybeMissing::Some(item) => fun(item),
        }
    }
}

impl<T: Sized> From<Option<T>> for MaybeMissing<T> {
    fn from(value: Option<T>) -> Self {
        value.map_or_else(|| Self::Missing, |inner| Self::Some(inner))
    }
}

impl<T: Sized> From<MaybeMissing<T>> for Option<T> {
    fn from(value: MaybeMissing<T>) -> Self {
        match value {
            MaybeMissing::Missing => None,
            MaybeMissing::Some(item) => Some(item),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Hash)]
pub struct SalsaEvent {
    pub query: MaybeMissing<String>,
    pub argument: MaybeMissing<u64>,
    pub kind: EventKind,
}

impl std::fmt::Display for SalsaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.query, self.kind)
    }
}

impl SalsaEvent {
    pub fn new(db: &AnalysisDatabase, event: &salsa::Event) -> Self {
        let (kind, key) = match event.kind {
            salsa::EventKind::DidValidateMemoizedValue { database_key } => {
                (EventKind::DidValidateMemoizedValue, Some(database_key))
            }
            salsa::EventKind::WillBlockOn { other_runtime_id: _, database_key } => {
                (EventKind::WillBlockOn, Some(database_key))
            }
            salsa::EventKind::WillExecute { database_key } => {
                (EventKind::WillExecute, Some(database_key))
            }
            salsa::EventKind::WillCheckCancellation => (EventKind::WillCheckCancellation, None),
        };

        match key {
            Some(key) => {
                let repr = format!("{:?}", key.debug(db));
                let query = MaybeMissing::Some(repr.split("(").next().unwrap().to_string());
                let argument = MaybeMissing::Some(short_hash(&repr));
                Self { query, argument, kind }
            }
            None => Self { kind, ..Default::default() },
        }
    }
}

fn short_hash(argument: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    argument.hash(&mut hasher);
    hasher.finish()
}
