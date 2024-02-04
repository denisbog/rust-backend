use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use chrono::NaiveDateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use sqlx::{error::BoxDynError, mysql::MySqlValueRef, Decode, Error, MySql};
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Point {
    pub lat: f64,
    pub lng: f64,
}
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DbItem {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_type: Option<String>,
    pub price: Option<f64>,
    pub location: Option<Point>,
    pub place_description: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub user: Option<String>,
    pub reserved: Option<String>,
    pub status: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl<'r> Decode<'r, MySql> for Point {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let bytes = <&[u8] as Decode<MySql>>::decode(value).unwrap();
        let mut cursor = std::io::Cursor::new(bytes);
        let _ = cursor.read_u32::<LittleEndian>();
        match cursor.read_u8() {
            Ok(1) => {
                let _ = cursor.read_u32::<LittleEndian>();
                Ok(Point {
                    lat: cursor.read_f64::<LittleEndian>().unwrap(),
                    lng: cursor.read_f64::<LittleEndian>().unwrap(),
                })
            }
            _ => Err(Error::Protocol("failed to read point from database".into()).into()),
        }
    }
}
