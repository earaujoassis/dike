use serde::de::Deserializer;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub update_datetime_stringdate: String,
    pub subdomain: String,
    pub remote_ip: String,
}
