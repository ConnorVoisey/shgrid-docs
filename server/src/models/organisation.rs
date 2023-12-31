use super::IndexQueryable;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organisation {
    pub id: Uuid,
    pub name: String,
    pub postcode: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganisationFields {
    Id,
    Name,
    Postcode,
    Active,
    CreatedAt,
    UpdatedAt,
}
impl IndexQueryable for Organisation {
    type Output = Organisation;
    type Fields = OrganisationFields;
    fn get_col_str_val(&self, key: &OrganisationFields) -> String {
        match key {
            OrganisationFields::Id => self.id.to_string(),
            OrganisationFields::Name => self.name.to_string(),
            OrganisationFields::Postcode => self.postcode.to_string(),
            OrganisationFields::Active => self.active.to_string(),
            OrganisationFields::CreatedAt => self.created_at.to_string(),
            OrganisationFields::UpdatedAt => self.updated_at.unwrap_or_default().to_string(),
        }
    }
}

/// Returns a constant vector of organisations,
/// This is the same everytime this function is called so that the output can be used in tests
pub fn create_organisations() -> Vec<Organisation> {
    vec![
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("ZephyrSoft Solutions"),
            postcode: String::from("59635-4901"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("QuantumStride Innovations"),
            postcode: String::from("03963"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("SparkWise Technologies"),
            postcode: String::from("59635-123"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("NovaPeak Enterprises"),
            postcode: String::from("7812-9823"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("CrestCore Dynamics"),
            postcode: String::from("82922-9876"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("NexusSphere Systems"),
            postcode: String::from("12345-6789"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("BlueFlare Innovations"),
            postcode: String::from("23788-2893"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("ZenithWave Solutions"),
            postcode: String::from("82389-2839"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("OrionEdge Technologies"),
            postcode: String::from("92192-2989"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
        Organisation {
            id: Uuid::from_u128(121040434418823839494894009918579823997),
            name: String::from("ApexLink Dynamics"),
            postcode: String::from("98239-9829"),
            active: false,
            created_at: DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .into(),
            updated_at: None,
        },
    ]
}
