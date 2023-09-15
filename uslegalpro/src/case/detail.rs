use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detail {
    pub case_tracking_id: String,
    pub jurisdiction: String,
    pub case_category: String,
    pub case_title: String,
    pub case_number: String,
    pub assigned_judge: String,
    pub case_type: String,
    pub case_category_display: String,
    pub jurisdiction_display: String,
    pub case_type_display: String,
    pub case_parties: Vec<CaseParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseParty {
    pub id: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub business_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub type_display: String,
    pub is_business: String,
    #[serde(alias = "type")]
    pub party_type: String,
}
