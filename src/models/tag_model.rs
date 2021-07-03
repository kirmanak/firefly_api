/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagModel {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The tag
    #[serde(rename = "tag")]
    pub tag: String,
    /// The date to which the tag is applicable.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Latitude of the tag's location, if applicable. Can be used to draw a map.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Latitude of the tag's location, if applicable. Can be used to draw a map.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Zoom level for the map, if drawn. This to set the box right. Unfortunately this is a proprietary value because each map provider has different zoom levels.
    #[serde(rename = "zoom_level", skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<i32>,
}

impl TagModel {
    pub fn new(tag: String) -> TagModel {
        TagModel {
            created_at: None,
            updated_at: None,
            tag,
            date: None,
            description: None,
            latitude: None,
            longitude: None,
            zoom_level: None,
        }
    }
}


