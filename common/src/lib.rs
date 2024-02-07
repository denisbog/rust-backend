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

impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for DbItem
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    Option<u64>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<u64>: ::sqlx::types::Type<R::Database>,
    Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<String>: ::sqlx::types::Type<R::Database>,
    Option<f64>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<f64>: ::sqlx::types::Type<R::Database>,
    Option<Point>: ::sqlx::decode::Decode<'a, R::Database>,
    // Option<Point>: ::sqlx::types::Type<R::Database>,
    Option<NaiveDateTime>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<NaiveDateTime>: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        let id: Option<u64> = row.try_get("id")?;
        let title: Option<String> = row.try_get("title")?;
        let description: Option<String> = row.try_get("description")?;
        let price_type: Option<String> = row.try_get("price_type")?;
        let price: Option<f64> = row.try_get("price")?;
        let location: Option<Point> = row.try_get_unchecked("location")?;
        let place_description: Option<String> = row.try_get("place_description")?;
        let category: Option<String> = row.try_get("category")?;
        let subcategory: Option<String> = row.try_get("subcategory")?;
        let user: Option<String> = row.try_get("user")?;
        let reserved: Option<String> = row.try_get("reserved")?;
        let status: Option<String> = row.try_get("status")?;
        let created: Option<NaiveDateTime> = row.try_get("created")?;
        let updated: Option<NaiveDateTime> = row.try_get("updated")?;
        ::std::result::Result::Ok(DbItem {
            id,
            title,
            description,
            price_type,
            price,
            location,
            place_description,
            category,
            subcategory,
            user,
            reserved,
            status,
            created,
            updated,
        })
    }
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
