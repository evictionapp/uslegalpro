//! the preview type. contains all the info necessary to determine if the case is what you are looking for plus a case_tracking_id to get more case details via
//! the [query](crate::case::query) method

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Preview {
    pub case_tracking_id: String,
    pub jurisdiction: String,
    pub case_category: String,
    pub case_number: String,
    pub case_title: String,
    pub case_type: String,
    pub case_category_display: String,
    pub jurisdiction_display: String,
    pub case_type_display: String,
}
