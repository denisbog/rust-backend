#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "/api";
pub const API_VERSION: &str = "2024-01-27T18:08:19Z";

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthorizedGetResponse {
    /// 302 response
    Status302
    {
        location:
        Option<
        String
        >
        ,
        set_cookie:
        Option<
        String
        >
    }
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsGetResponse {
    /// 200 response
    Status200
    (models::Items)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdContentGetResponse {
    /// 200 response
    Status200
    (ByteArray)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdContentPutResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdDeleteResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdGetResponse {
    /// 200 response
    Status200
    (models::Item)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdPostResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsPutResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LoginGetResponse {
    /// 302 response
    Status302
    {
        location:
        Option<
        String
        >
    }
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MyItemsGetResponse {
    /// 200 response
    Status200
    (models::Items)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MyRelatedGetResponse {
    /// 200 response
    Status200
    (models::Items)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsGetResponse {
    /// 200 response
    Status200
    (models::Reservations)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdAcceptPostResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdDeclinePostResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdDeleteResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdGetResponse {
    /// 200 response
    Status200
    (models::Reservation)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdPostResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsIdReturnPostResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReservationsPutResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SearchGetResponse {
    /// 200 response
    Status200
    (models::Items)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersGetResponse {
    /// 200 response
    Status200
    (models::Users)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersIdDeleteResponse {
    /// 200 response
    Status200
    (String)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersIdGetResponse {
    /// 200 response
    Status200
    (models::User)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersIdPostResponse {
    /// 200 response
    Status200
    (String)
}


/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {

                /// AuthorizedGet - GET /api/authorized
                async fn authorized_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::AuthorizedGetQueryParams,
                ) -> Result<AuthorizedGetResponse, String>;


                /// ItemsGet - GET /api/items
                async fn items_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::ItemsGetQueryParams,
                ) -> Result<ItemsGetResponse, String>;


                /// ItemsIdContentGet - GET /api/items/{id}/content
                async fn items_id_content_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ItemsIdContentGetPathParams,
                ) -> Result<ItemsIdContentGetResponse, String>;


                /// ItemsIdContentPut - PUT /api/items/{id}/content
                async fn items_id_content_put(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ItemsIdContentPutPathParams,
                        body: Bytes,
                ) -> Result<ItemsIdContentPutResponse, String>;


                /// ItemsIdDelete - DELETE /api/items/{id}
                async fn items_id_delete(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ItemsIdDeletePathParams,
                ) -> Result<ItemsIdDeleteResponse, String>;


                /// ItemsIdGet - GET /api/items/{id}
                async fn items_id_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ItemsIdGetPathParams,
                ) -> Result<ItemsIdGetResponse, String>;


                /// ItemsIdPost - POST /api/items/{id}
                async fn items_id_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ItemsIdPostPathParams,
                        body: models::Item,
                ) -> Result<ItemsIdPostResponse, String>;


                /// ItemsPut - PUT /api/items
                async fn items_put(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: models::Item,
                ) -> Result<ItemsPutResponse, String>;


                /// LoginGet - GET /api/login
                async fn login_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<LoginGetResponse, String>;


                /// MyItemsGet - GET /api/my-items
                async fn my_items_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::MyItemsGetQueryParams,
                ) -> Result<MyItemsGetResponse, String>;


                /// MyRelatedGet - GET /api/my-related
                async fn my_related_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::MyRelatedGetQueryParams,
                ) -> Result<MyRelatedGetResponse, String>;


                /// ReservationsGet - GET /api/reservations
                async fn reservations_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::ReservationsGetQueryParams,
                ) -> Result<ReservationsGetResponse, String>;


                /// ReservationsIdAcceptPost - POST /api/reservations/{id}/accept
                async fn reservations_id_accept_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdAcceptPostPathParams,
                ) -> Result<ReservationsIdAcceptPostResponse, String>;


                /// ReservationsIdDeclinePost - POST /api/reservations/{id}/decline
                async fn reservations_id_decline_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdDeclinePostPathParams,
                ) -> Result<ReservationsIdDeclinePostResponse, String>;


                /// ReservationsIdDelete - DELETE /api/reservations/{id}
                async fn reservations_id_delete(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdDeletePathParams,
                ) -> Result<ReservationsIdDeleteResponse, String>;


                /// ReservationsIdGet - GET /api/reservations/{id}
                async fn reservations_id_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdGetPathParams,
                ) -> Result<ReservationsIdGetResponse, String>;


                /// ReservationsIdPost - POST /api/reservations/{id}
                async fn reservations_id_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdPostPathParams,
                        body: models::Reservation,
                ) -> Result<ReservationsIdPostResponse, String>;


                /// ReservationsIdReturnPost - POST /api/reservations/{id}/return
                async fn reservations_id_return_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ReservationsIdReturnPostPathParams,
                ) -> Result<ReservationsIdReturnPostResponse, String>;


                /// ReservationsPut - PUT /api/reservations
                async fn reservations_put(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: models::Reservation,
                ) -> Result<ReservationsPutResponse, String>;


                /// SearchGet - GET /api/search
                async fn search_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::SearchGetQueryParams,
                ) -> Result<SearchGetResponse, String>;


                /// UsersGet - GET /api/users
                async fn users_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::UsersGetQueryParams,
                ) -> Result<UsersGetResponse, String>;


                /// UsersIdDelete - DELETE /api/users/{id}
                async fn users_id_delete(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::UsersIdDeletePathParams,
                ) -> Result<UsersIdDeleteResponse, String>;


                /// UsersIdGet - GET /api/users/{id}
                async fn users_id_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::UsersIdGetPathParams,
                ) -> Result<UsersIdGetResponse, String>;


                /// UsersIdPost - POST /api/users/{id}
                async fn users_id_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::UsersIdPostPathParams,
                ) -> Result<UsersIdPostResponse, String>;

}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
