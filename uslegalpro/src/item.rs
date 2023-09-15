//! Common types used by uslegalpro for communication. they are DTO objects.

use serde::{Deserialize, Serialize};

/// a response that contains only one type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item<T> {
    pub item: T,
}

/// a response that contains more than one type or zero
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Items<T> {
    pub items: Vec<T>,
}

/// a request that is post commonly requires you to wrap your type in "data"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}
