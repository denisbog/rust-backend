#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct AuthorizedGetHeaderParams {
        pub authorization: Option<String>,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct AuthorizedGetQueryParams {
                #[serde(rename = "code")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub code: Option<String>,
    }

      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsContentNameDeletePathParams {
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsContentNameGetPathParams {
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsContentNamePutPathParams {
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsGetQueryParams {
                #[serde(rename = "LastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key: Option<String>,
                #[serde(rename = "category")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub category: Option<String>,
                #[serde(rename = "subcategory")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub subcategory: Option<String>,
                #[serde(rename = "lat")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub lat: Option<f64>,
                #[serde(rename = "long")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub long: Option<f64>,
                #[serde(rename = "r")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub r: Option<f64>,
                #[serde(rename = "q")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub q: Option<String>,
                #[serde(rename = "lastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key2: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdContentNameDeletePathParams {
                pub id: String,
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdContentNameGetPathParams {
                pub id: String,
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdContentNamePutPathParams {
                pub id: String,
                pub name: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdDeletePathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdGetPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ItemsIdPostPathParams {
                pub id: String,
    }


      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MyItemsGetQueryParams {
                #[serde(rename = "LastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MyRelatedGetQueryParams {
                #[serde(rename = "LastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsGetQueryParams {
                #[serde(rename = "LastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdAcceptPostPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdDeclinePostPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdDeletePathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdGetPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdPostPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ReservationsIdReturnPostPathParams {
                pub id: String,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct SearchGetQueryParams {
                #[serde(rename = "text")]
                pub text: String,
                #[serde(rename = "category")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub category: Option<String>,
                #[serde(rename = "subcategory")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub subcategory: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersGetQueryParams {
                #[serde(rename = "LastEvaluatedKey")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub last_evaluated_key: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersIdDeletePathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersIdGetPathParams {
                pub id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct UsersIdPostPathParams {
                pub id: String,
    }







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Item {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "updated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "priceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub price_type: Option<String>,

    #[serde(rename = "price")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub price: Option<f64>,

    #[serde(rename = "place")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub place: Option<models::ItemPlace>,

    #[serde(rename = "category")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub category: Option<String>,

    #[serde(rename = "subcategory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subcategory: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "reserved")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reserved: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_email: Option<String>,

    #[serde(rename = "userAvatar")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_avatar: Option<String>,

    #[serde(rename = "image")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "images")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<Vec<String>>,

}


impl Item {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Item {
        Item {
            id: None,
            title: None,
            description: None,
            created: None,
            updated: None,
            price_type: None,
            price: None,
            place: None,
            category: None,
            subcategory: None,
            user: None,
            reserved: None,
            status: None,
            user_name: None,
            user_email: None,
            user_avatar: None,
            image: None,
            images: None,
        }
    }
}

/// Converts the Item value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Item {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.title.as_ref().map(|title| {
                [
                    "title".to_string(),
                    title.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                [
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

            // Skipping created in query parameter serialization

            // Skipping updated in query parameter serialization


            self.price_type.as_ref().map(|price_type| {
                [
                    "priceType".to_string(),
                    price_type.to_string(),
                ].join(",")
            }),


            self.price.as_ref().map(|price| {
                [
                    "price".to_string(),
                    price.to_string(),
                ].join(",")
            }),

            // Skipping place in query parameter serialization


            self.category.as_ref().map(|category| {
                [
                    "category".to_string(),
                    category.to_string(),
                ].join(",")
            }),


            self.subcategory.as_ref().map(|subcategory| {
                [
                    "subcategory".to_string(),
                    subcategory.to_string(),
                ].join(",")
            }),


            self.user.as_ref().map(|user| {
                [
                    "user".to_string(),
                    user.to_string(),
                ].join(",")
            }),


            self.reserved.as_ref().map(|reserved| {
                [
                    "reserved".to_string(),
                    reserved.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.user_name.as_ref().map(|user_name| {
                [
                    "userName".to_string(),
                    user_name.to_string(),
                ].join(",")
            }),


            self.user_email.as_ref().map(|user_email| {
                [
                    "userEmail".to_string(),
                    user_email.to_string(),
                ].join(",")
            }),


            self.user_avatar.as_ref().map(|user_avatar| {
                [
                    "userAvatar".to_string(),
                    user_avatar.to_string(),
                ].join(",")
            }),


            self.image.as_ref().map(|image| {
                [
                    "image".to_string(),
                    image.to_string(),
                ].join(",")
            }),


            self.images.as_ref().map(|images| {
                [
                    "images".to_string(),
                    images.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Item value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub title: Vec<String>,
            pub description: Vec<String>,
            pub created: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated: Vec<chrono::DateTime::<chrono::Utc>>,
            pub price_type: Vec<String>,
            pub price: Vec<f64>,
            pub place: Vec<models::ItemPlace>,
            pub category: Vec<String>,
            pub subcategory: Vec<String>,
            pub user: Vec<String>,
            pub reserved: Vec<String>,
            pub status: Vec<String>,
            pub user_name: Vec<String>,
            pub user_email: Vec<String>,
            pub user_avatar: Vec<String>,
            pub image: Vec<String>,
            pub images: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Item".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created" => intermediate_rep.created.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated" => intermediate_rep.updated.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "priceType" => intermediate_rep.price_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "place" => intermediate_rep.place.push(<models::ItemPlace as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "category" => intermediate_rep.category.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "subcategory" => intermediate_rep.subcategory.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "user" => intermediate_rep.user.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reserved" => intermediate_rep.reserved.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userName" => intermediate_rep.user_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userEmail" => intermediate_rep.user_email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userAvatar" => intermediate_rep.user_avatar.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "image" => intermediate_rep.image.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "images" => return std::result::Result::Err("Parsing a container in this style is not supported in Item".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Item".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Item {
            id: intermediate_rep.id.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            created: intermediate_rep.created.into_iter().next(),
            updated: intermediate_rep.updated.into_iter().next(),
            price_type: intermediate_rep.price_type.into_iter().next(),
            price: intermediate_rep.price.into_iter().next(),
            place: intermediate_rep.place.into_iter().next(),
            category: intermediate_rep.category.into_iter().next(),
            subcategory: intermediate_rep.subcategory.into_iter().next(),
            user: intermediate_rep.user.into_iter().next(),
            reserved: intermediate_rep.reserved.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            user_name: intermediate_rep.user_name.into_iter().next(),
            user_email: intermediate_rep.user_email.into_iter().next(),
            user_avatar: intermediate_rep.user_avatar.into_iter().next(),
            image: intermediate_rep.image.into_iter().next(),
            images: intermediate_rep.images.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Item> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Item>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Item>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Item - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Item> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Item as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Item - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ItemPlace {
    #[serde(rename = "lat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lat: Option<f64>,

    #[serde(rename = "lng")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lng: Option<f64>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}


impl ItemPlace {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ItemPlace {
        ItemPlace {
            lat: None,
            lng: None,
            description: None,
        }
    }
}

/// Converts the ItemPlace value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ItemPlace {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.lat.as_ref().map(|lat| {
                [
                    "lat".to_string(),
                    lat.to_string(),
                ].join(",")
            }),


            self.lng.as_ref().map(|lng| {
                [
                    "lng".to_string(),
                    lng.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                [
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ItemPlace value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ItemPlace {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub lat: Vec<f64>,
            pub lng: Vec<f64>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ItemPlace".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "lat" => intermediate_rep.lat.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lng" => intermediate_rep.lng.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ItemPlace".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ItemPlace {
            lat: intermediate_rep.lat.into_iter().next(),
            lng: intermediate_rep.lng.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ItemPlace> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ItemPlace>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ItemPlace>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ItemPlace - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ItemPlace> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ItemPlace as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ItemPlace - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Items {
    #[serde(rename = "Items")]
    pub items: Vec<models::Item>,

    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_evaluated_key: Option<String>,

}


impl Items {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(items: Vec<models::Item>, ) -> Items {
        Items {
            items,
            last_evaluated_key: None,
        }
    }
}

/// Converts the Items value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Items {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping Items in query parameter serialization


            self.last_evaluated_key.as_ref().map(|last_evaluated_key| {
                [
                    "LastEvaluatedKey".to_string(),
                    last_evaluated_key.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Items value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Items {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::Item>>,
            pub last_evaluated_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Items".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "Items" => return std::result::Result::Err("Parsing a container in this style is not supported in Items".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "LastEvaluatedKey" => intermediate_rep.last_evaluated_key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Items".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Items {
            items: intermediate_rep.items.into_iter().next().ok_or_else(|| "Items missing in Items".to_string())?,
            last_evaluated_key: intermediate_rep.last_evaluated_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Items> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Items>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Items>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Items - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Items> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Items as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Items - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Reservation {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "item")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub item: Option<String>,

    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_email: Option<String>,

    #[serde(rename = "userAvatar")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_avatar: Option<String>,

}


impl Reservation {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Reservation {
        Reservation {
            id: None,
            message: None,
            created: None,
            item: None,
            user_name: None,
            user_email: None,
            user_avatar: None,
        }
    }
}

/// Converts the Reservation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Reservation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.message.as_ref().map(|message| {
                [
                    "message".to_string(),
                    message.to_string(),
                ].join(",")
            }),

            // Skipping created in query parameter serialization


            self.item.as_ref().map(|item| {
                [
                    "item".to_string(),
                    item.to_string(),
                ].join(",")
            }),


            self.user_name.as_ref().map(|user_name| {
                [
                    "userName".to_string(),
                    user_name.to_string(),
                ].join(",")
            }),


            self.user_email.as_ref().map(|user_email| {
                [
                    "userEmail".to_string(),
                    user_email.to_string(),
                ].join(",")
            }),


            self.user_avatar.as_ref().map(|user_avatar| {
                [
                    "userAvatar".to_string(),
                    user_avatar.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Reservation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Reservation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub message: Vec<String>,
            pub created: Vec<chrono::DateTime::<chrono::Utc>>,
            pub item: Vec<String>,
            pub user_name: Vec<String>,
            pub user_email: Vec<String>,
            pub user_avatar: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Reservation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created" => intermediate_rep.created.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "item" => intermediate_rep.item.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userName" => intermediate_rep.user_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userEmail" => intermediate_rep.user_email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userAvatar" => intermediate_rep.user_avatar.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Reservation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Reservation {
            id: intermediate_rep.id.into_iter().next(),
            message: intermediate_rep.message.into_iter().next(),
            created: intermediate_rep.created.into_iter().next(),
            item: intermediate_rep.item.into_iter().next(),
            user_name: intermediate_rep.user_name.into_iter().next(),
            user_email: intermediate_rep.user_email.into_iter().next(),
            user_avatar: intermediate_rep.user_avatar.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Reservation> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Reservation>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Reservation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Reservation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Reservation> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Reservation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Reservation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Reservations {
    #[serde(rename = "Items")]
    pub items: Vec<models::Reservation>,

    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_evaluated_key: Option<String>,

}


impl Reservations {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(items: Vec<models::Reservation>, ) -> Reservations {
        Reservations {
            items,
            last_evaluated_key: None,
        }
    }
}

/// Converts the Reservations value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Reservations {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping Items in query parameter serialization


            self.last_evaluated_key.as_ref().map(|last_evaluated_key| {
                [
                    "LastEvaluatedKey".to_string(),
                    last_evaluated_key.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Reservations value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Reservations {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::Reservation>>,
            pub last_evaluated_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Reservations".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "Items" => return std::result::Result::Err("Parsing a container in this style is not supported in Reservations".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "LastEvaluatedKey" => intermediate_rep.last_evaluated_key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Reservations".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Reservations {
            items: intermediate_rep.items.into_iter().next().ok_or_else(|| "Items missing in Reservations".to_string())?,
            last_evaluated_key: intermediate_rep.last_evaluated_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Reservations> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Reservations>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Reservations>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Reservations - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Reservations> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Reservations as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Reservations - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct User {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "about")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub about: Option<String>,

    #[serde(rename = "avatar")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "joined")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub joined: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "lastLogin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_login: Option<chrono::DateTime::<chrono::Utc>>,

}


impl User {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> User {
        User {
            id: None,
            name: None,
            about: None,
            avatar: None,
            email: None,
            joined: None,
            last_login: None,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for User {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.about.as_ref().map(|about| {
                [
                    "about".to_string(),
                    about.to_string(),
                ].join(",")
            }),


            self.avatar.as_ref().map(|avatar| {
                [
                    "avatar".to_string(),
                    avatar.to_string(),
                ].join(",")
            }),


            self.email.as_ref().map(|email| {
                [
                    "email".to_string(),
                    email.to_string(),
                ].join(",")
            }),

            // Skipping joined in query parameter serialization

            // Skipping lastLogin in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub about: Vec<String>,
            pub avatar: Vec<String>,
            pub email: Vec<String>,
            pub joined: Vec<chrono::DateTime::<chrono::Utc>>,
            pub last_login: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "about" => intermediate_rep.about.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "avatar" => intermediate_rep.avatar.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "joined" => intermediate_rep.joined.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastLogin" => intermediate_rep.last_login.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            about: intermediate_rep.about.into_iter().next(),
            avatar: intermediate_rep.avatar.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            joined: intermediate_rep.joined.into_iter().next(),
            last_login: intermediate_rep.last_login.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Users {
    #[serde(rename = "Items")]
    pub items: Vec<models::User>,

    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_evaluated_key: Option<String>,

}


impl Users {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(items: Vec<models::User>, ) -> Users {
        Users {
            items,
            last_evaluated_key: None,
        }
    }
}

/// Converts the Users value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Users {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping Items in query parameter serialization


            self.last_evaluated_key.as_ref().map(|last_evaluated_key| {
                [
                    "LastEvaluatedKey".to_string(),
                    last_evaluated_key.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Users value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Users {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::User>>,
            pub last_evaluated_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Users".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "Items" => return std::result::Result::Err("Parsing a container in this style is not supported in Users".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "LastEvaluatedKey" => intermediate_rep.last_evaluated_key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Users".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Users {
            items: intermediate_rep.items.into_iter().next().ok_or_else(|| "Items missing in Users".to_string())?,
            last_evaluated_key: intermediate_rep.last_evaluated_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Users> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Users>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Users>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Users - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Users> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Users as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Users - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



