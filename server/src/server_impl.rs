use std::ops::Add;

use crate::ServerImpl;
use ::chrono::Duration;
use async_session::log::info;
use async_session::Session;
use async_session::SessionStore;
use async_trait::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use common::DbItem;
use common::DbUser;
use common::Point;
use http::Method;
use oauth2::reqwest::async_http_client;
use oauth2::TokenResponse;
use openapi::models::Item;
use openapi::models::ItemPlace;
use openapi::models::Reservation;
use openapi::models::Reservations;
use openapi::models::Users;
use openapi::models::{self, User};
use openapi::types::ByteArray;
use openapi::AuthorizedGetResponse;
use openapi::AuthorizedPostResponse;
use openapi::ItemsContentGetResponse;
use openapi::ItemsContentNameDeleteResponse;
use openapi::ItemsContentNameGetResponse;
use openapi::ItemsContentNamePutResponse;
use openapi::ItemsIdContentNameDeleteResponse;
use openapi::ItemsIdContentNameGetResponse;
use openapi::ItemsIdContentNamePutResponse;
use openapi::ItemsIdReservationsGetResponse;
use openapi::SearchGetResponse;
use openapi::{
    ItemsGetResponse, ItemsIdDeleteResponse, ItemsIdGetResponse, ItemsIdPostResponse,
    ItemsPutResponse, MyItemsGetResponse, MyRelatedGetResponse, ReservationsGetResponse,
    ReservationsIdAcceptPostResponse, ReservationsIdDeclinePostResponse,
    ReservationsIdDeleteResponse, ReservationsIdGetResponse, ReservationsIdPostResponse,
    ReservationsIdReturnPostResponse, ReservationsPutResponse, UsersGetResponse,
    UsersIdDeleteResponse, UsersIdGetResponse, UsersIdPostResponse,
};

use oauth2::AuthorizationCode;
use sqlx::types::chrono;

#[allow(unused_variables)]
#[async_trait]
impl openapi::Api for ServerImpl {
    #[doc = r" AuthorizedGet - GET /api/authorized"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn authorized_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::AuthorizedGetHeaderParams,
        query_params: models::AuthorizedGetQueryParams,
    ) -> Result<AuthorizedGetResponse, String> {
        let token = if header_params.authorization.is_none() {
            self.oauth_client
                .exchange_code(AuthorizationCode::new(query_params.code.unwrap()))
                .request_async(async_http_client)
                .await
                .unwrap()
                .access_token()
                .secret()
                .to_owned()
        } else {
            header_params
                .authorization
                .unwrap()
                .strip_prefix("Bearer ")
                .unwrap()
                .to_string()
        };
        let (user_data, user_json) = Self::get_user_data_for_token(&token).await;
        let mut session = Session::new();
        session.set_expiry(chrono::Utc::now().add(Duration::days(10)));
        session.insert("user", &user_data).unwrap();
        self.check_and_import_user_if_required(&user_data).await;

        // Store session and get corresponding cookie
        let cookie = &self.store.store_session(session).await.unwrap().unwrap();
        // if let Some(redirect_uri) = query_params.redirect_uri {
        //     tracing::info!("redirect_uri {}", redirect_uri);
        //     Ok(openapi::AuthorizedGetResponse::Status302 {
        //         location: Some(redirect_uri),
        //         set_cookie: Some(format!("session={cookie}; SameSite=Lax; Path=/")),
        //     })
        // } else {
        Ok(openapi::AuthorizedGetResponse::Status200 {
            set_cookie: Some(format!(
                "session={cookie}; SameSite=None; Secure; Path=/api; Max-Age=864000"
            )),
            body: format!(
                r#"
<html>
<head></head>
<body>
  <script>
    window.addEventListener("message", function (event) {{
      if (event.data.message === "requestResult") {{
        event.source.postMessage({{"message": "deliverResult", result: {{"me":{user_json}, "access_token": "{cookie}" }} }}, "*");
      }}
    }});
  </script>
</body>
</html>
                "#
            ),
        })
        // }
    }

    #[doc = r" AuthorizedPost - POST /api/authorized"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn authorized_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::AuthorizedPostHeaderParams,
        body: models::Authorized,
    ) -> Result<AuthorizedPostResponse, String> {
        tracing::info!("{:?}", body);
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(body.code.unwrap()))
            .request_async(async_http_client)
            .await
            .unwrap()
            .access_token()
            .secret()
            .to_owned();

        let client = reqwest::Client::new();

        let user_json: serde_json::Value = client
            .get("https://graph.facebook.com/me?fields=name,first_name,last_name,email,picture")
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let user_data = User {
            avatar: Some(
                user_json
                    .get("picture")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("data")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("url")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            ),
            email: Some(
                user_json
                    .get("email")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            ),
            phone: None,
            about: None,
            id: Some(user_json.get("id").unwrap().as_str().unwrap().to_string()),
            last_login: None,
            joined: None,
            name: Some(user_json.get("name").unwrap().as_str().unwrap().to_string()),
        };

        let mut session = Session::new();
        session.set_expiry(chrono::Utc::now().add(Duration::days(10)));
        session.insert("user", &user_data).unwrap();
        self.check_and_import_user_if_required(&user_data).await;

        // Store session and get corresponding cookie
        let cookie = &self.store.store_session(session).await.unwrap().unwrap();
        // if let Some(redirect_uri) = body.redirect_uri {
        //     tracing::info!("redirect_uri {}", redirect_uri);
        //     Ok(openapi::AuthorizedPostResponse::Status302 {
        //         location: Some(redirect_uri),
        //         set_cookie: Some(format!("session={cookie}; SameSite=Lax; Path=/")),
        //     })
        // } else {
        Ok(openapi::AuthorizedPostResponse::Status200 {
            set_cookie: Some(format!(
                "session={cookie}; SameSite=None; Secure; Path=/api; Max-Age=864000"
            )),
            body: format!(r#"{{"me":{user_json}, "access_token": "{cookie}" }}"#),
        })
        // }
    }

    #[doc = r" ItemsContentGet - GET /api/items/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_content_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsContentGetHeaderParams,
    ) -> Result<ItemsContentGetResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            if let Some(contents) = self.get_images_for_item(&current_user_id).await {
                Ok(ItemsContentGetResponse::Status200(contents))
            } else {
                Err("no content found".to_string())
            }
        } else {
            Ok(ItemsContentGetResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsContentNameDelete - DELETE /api/items/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_content_name_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsContentNameDeleteHeaderParams,
        path_params: models::ItemsContentNameDeletePathParams,
    ) -> Result<ItemsContentNameDeleteResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            self.delete_content(&current_user_id, &path_params.name)
                .await
                .unwrap();
            Ok(ItemsContentNameDeleteResponse::Status200("ok".to_string()))
        } else {
            Ok(ItemsContentNameDeleteResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsContentNameGet - GET /api/items/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_content_name_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsContentNameGetHeaderParams,
        path_params: models::ItemsContentNameGetPathParams,
    ) -> Result<ItemsContentNameGetResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            if let Some(contents) = self.get_content(&current_user_id, &path_params.name).await {
                Ok(ItemsContentNameGetResponse::Status200(ByteArray(contents)))
            } else {
                Ok(ItemsContentNameGetResponse::Status401(
                    "file not found".to_string(),
                ))
            }
        } else {
            Ok(ItemsContentNameGetResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsContentNamePut - PUT /api/items/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_content_name_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsContentNamePutHeaderParams,
        path_params: models::ItemsContentNamePutPathParams,
        body: Bytes,
    ) -> Result<ItemsContentNamePutResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            self.upload_content(&current_user_id, &path_params.name, &body)
                .await;
            Ok(ItemsContentNamePutResponse::Status200("ok".to_string()))
        } else {
            Ok(ItemsContentNamePutResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsGet - GET /api/items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ItemsGetQueryParams,
    ) -> Result<ItemsGetResponse, String> {
        let test = cookies.get("session");

        tracing::info!("test {:?}", test);
        let (items, last_evaluated_key) = self
            .get_items(
                query_params.category,
                query_params.subcategory,
                query_params.lat,
                query_params.long,
                query_params.r,
                query_params.q,
                None,
                None,
                query_params.last_evaluated_key,
            )
            .await;

        Ok(openapi::ItemsGetResponse::Status200(
            openapi::models::Items {
                items,
                last_evaluated_key,
            },
        ))
    }

    #[doc = r" ItemsIdContentNameDelete - DELETE /api/items/{id}/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_name_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsIdContentNameDeleteHeaderParams,
        path_params: models::ItemsIdContentNameDeletePathParams,
    ) -> Result<ItemsIdContentNameDeleteResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            self.delete_content(&path_params.id, &path_params.name)
                .await
                .unwrap();
            Ok(ItemsIdContentNameDeleteResponse::Status200(
                "ok".to_string(),
            ))
        } else {
            Ok(ItemsIdContentNameDeleteResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsIdContentGet - GET /api/items/{id}/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_name_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdContentNameGetPathParams,
    ) -> Result<ItemsIdContentNameGetResponse, String> {
        if let Some(contents) = self.get_content(&path_params.id, &path_params.name).await {
            Ok(ItemsIdContentNameGetResponse::Status200(ByteArray(
                contents,
            )))
        } else {
            Ok(ItemsIdContentNameGetResponse::Status401(
                "file not found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsIdContentPut - PUT /api/items/{id}/content/{name}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_name_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsIdContentNamePutHeaderParams,
        path_params: models::ItemsIdContentNamePutPathParams,
        body: Bytes,
    ) -> Result<ItemsIdContentNamePutResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            self.upload_content(&path_params.id, &path_params.name, &body)
                .await;
            Ok(ItemsIdContentNamePutResponse::Status200("ok".to_string()))
        } else {
            Ok(ItemsIdContentNamePutResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsIdDelete - DELETE /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsIdDeleteHeaderParams,
        path_params: models::ItemsIdDeletePathParams,
    ) -> Result<ItemsIdDeleteResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows = sqlx::query!(
                "delete from items where id = ? and user = ?",
                path_params.id,
                current_user_id
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();
            if rows > 0 {
                self.search_engine
                    .delete(path_params.id.parse::<u64>().unwrap())
                    .await;

                Ok(ItemsIdDeleteResponse::Status200(rows.to_string()))
            } else {
                Ok(ItemsIdDeleteResponse::Status403("forbidden".to_string()))
            }
        } else {
            Ok(ItemsIdDeleteResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsIdGet - GET /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdGetPathParams,
    ) -> Result<ItemsIdGetResponse, String> {
        let row = sqlx::query!(
            r#"SELECT
        items.id,
        title,
        description,
        created,
        updated,
        price_type,
        price,
        location as "location: Point",
        place_description,
        category,
        subcategory,
        image,
        user,
        reserved,
        status,
        name,
        email,
        avatar
    FROM 
        items 
    LEFT JOIN
        users
    ON
        items.user = users.id
    WHERE
        items.id = ?"#,
            path_params.id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();
        if let Some(row) = row {
            Ok(openapi::ItemsIdGetResponse::Status200(
                openapi::models::Item {
                    id: Some(row.id.to_string()),
                    title: Some(row.title),
                    description: Some(row.description),

                    created: Some(row.created.and_utc()),
                    updated: Some(row.updated.and_utc()),

                    price_type: Some(row.price_type),
                    price: Some(row.price),

                    place: Some(ItemPlace {
                        lat: Some(row.location.lat),
                        lng: Some(row.location.lng),
                        description: row.place_description,
                    }),
                    category: Some(row.category),
                    subcategory: Some(row.subcategory),
                    user: Some(row.user),
                    reserved: row.reserved,
                    status: row.status,
                    user_name: row.name,
                    user_email: row.email,
                    user_avatar: row.avatar,
                    image: row.image,
                    images: self.get_images_for_item(&path_params.id).await,
                },
            ))
        } else {
            Ok(openapi::ItemsIdGetResponse::Status200(
                openapi::models::Item::new(),
            ))
        }
    }

    #[doc = r" ItemsIdPost - POST /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsIdPostHeaderParams,
        path_params: models::ItemsIdPostPathParams,
        body: models::Item,
    ) -> Result<ItemsIdPostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let item_id = sqlx::query_scalar!(
                r#"
    UPDATE items
    SET
        title = ?,
        description = ?,
        price_type = ?,
        price = ?,
        location = Point(?,?),
        place_description = ?,
        category = ?,
        subcategory = ?,
        user = ?,
        reserved = ?,
        image = ?,
        status = ?
    WHERE
        id = ?
            "#,
                body.title,
                body.description,
                body.price_type,
                body.price,
                body.place.as_ref().unwrap().lat,
                body.place.as_ref().unwrap().lng,
                body.place.as_ref().unwrap().description,
                body.category,
                body.subcategory,
                current_user_id,
                body.reserved,
                body.image,
                body.status,
                path_params.id
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

            self.search_engine
                .index(&DbItem {
                    id: Some(path_params.id.parse::<u64>().unwrap()),
                    title: body.title,
                    description: body.description,
                    category: body.category,
                    subcategory: body.subcategory,
                    price_type: None,
                    price: None,
                    image: None,
                    created: None,
                    updated: None,
                    user: None,
                    reserved: None,
                    status: None,
                    location: None,
                    place_description: None,
                })
                .await;
            Ok(ItemsIdPostResponse::Status200(item_id.to_string()))
        } else {
            Ok(ItemsIdPostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsPut - PUT /api/items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ItemsPutHeaderParams,
        body: models::Item,
    ) -> Result<ItemsPutResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            if let Ok(mut transaction) = self.pool.begin().await {
                let item_id = sqlx::query_scalar!(
                    r#"
    INSERT INTO items(
        title,
        description,
        price_type,
        price,
        location,
        place_description,
        category,
        subcategory,
        image,
        user,
        reserved,
        status
        )
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?, ?, ?, ?)
            "#,
                    body.title,
                    body.description,
                    body.price_type,
                    body.price,
                    body.place.as_ref().unwrap().lat,
                    body.place.as_ref().unwrap().lng,
                    body.place.as_ref().unwrap().description,
                    body.category,
                    body.subcategory,
                    body.image,
                    current_user_id,
                    body.reserved,
                    body.status
                )
                .execute(&mut *transaction)
                .await
                .unwrap()
                .last_insert_id();

                if self
                    .move_images(&current_user_id, &item_id.to_string())
                    .await
                    .is_err()
                {
                    tracing::info!("failed moving folder from user session to item {item_id}, most probably no photos uploaded")
                }
                let _ = transaction.commit().await;

                self.search_engine
                    .index(&DbItem {
                        id: Some(item_id),
                        title: body.title,
                        description: body.description,
                        category: body.category,
                        subcategory: body.subcategory,
                        price_type: None,
                        price: None,
                        image: None,
                        created: None,
                        updated: None,
                        user: None,
                        reserved: None,
                        status: None,
                        location: None,
                        place_description: None,
                    })
                    .await;

                Ok(ItemsPutResponse::Status200(format!("{item_id}")))
            } else {
                Ok(ItemsPutResponse::Status401(
                    "failed to get transaction".to_string(),
                ))
            }
        } else {
            Ok(ItemsPutResponse::Status401("no session found".to_string()))
        }
    }

    #[doc = r" MyItemsGet - GET /api/my-items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn my_items_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::MyItemsGetHeaderParams,
        query_params: models::MyItemsGetQueryParams,
    ) -> Result<MyItemsGetResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let (items, last_evaluated_key) = self
                .get_items(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(current_user_id),
                    None,
                    query_params.last_evaluated_key,
                )
                .await;

            Ok(openapi::MyItemsGetResponse::Status200(
                openapi::models::Items {
                    items,
                    last_evaluated_key,
                },
            ))
        } else {
            Ok(MyItemsGetResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" MyRelatedGet - GET /api/my-related"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn my_related_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::MyRelatedGetHeaderParams,
        query_params: models::MyRelatedGetQueryParams,
    ) -> Result<MyRelatedGetResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let (items, last_evaluated_key) = self
                .get_items(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(current_user_id),
                    query_params.last_evaluated_key,
                )
                .await;

            Ok(openapi::MyRelatedGetResponse::Status200(
                openapi::models::Items {
                    items,
                    last_evaluated_key,
                },
            ))
        } else {
            Ok(MyRelatedGetResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsGet - GET /api/reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsGetHeaderParams,
        query_params: models::ReservationsGetQueryParams,
    ) -> Result<ReservationsGetResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let items = sqlx::query!(
                r#"select
                        reservations.id, item, message, reservations.created, name, avatar, email
                    from
                        reservations
                    join
                        items
                    on
                        item = items.id
                    left join
                        users
                    on
                        reservations.user = users.id
                    where
                        items.user = ?"#,
                current_user_id
            )
            .fetch_all(&self.pool)
            .await
            .unwrap();
            let items = items
                .into_iter()
                .map(|item| Reservation {
                    id: Some(item.id.to_string()),
                    item: Some(item.item.to_string()),
                    message: Some(item.message),
                    user_name: item.name,
                    user_avatar: item.avatar,
                    user_email: item.email,
                    created: Some(item.created.and_utc()),
                })
                .collect::<Vec<Reservation>>();
            Ok(ReservationsGetResponse::Status200(Reservations::new(items)))
        } else {
            Ok(ReservationsGetResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsIdAcceptPost - POST /api/reservations/{id}/accept"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_accept_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsIdAcceptPostHeaderParams,
        path_params: models::ReservationsIdAcceptPostPathParams,
    ) -> Result<ReservationsIdAcceptPostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let reservation = sqlx::query!(
                "select user, item from reservations where id = ?",
                path_params.id
            )
            .fetch_optional(&self.pool)
            .await
            .unwrap();
            if let Some(reservation) = reservation {
                if let Ok(mut transaction) = self.pool.begin().await {
                    let rows = sqlx::query!(
                        "update items set reserved = ?, status = ? where id = ? and user = ?",
                        reservation.user,
                        "rented",
                        reservation.item,
                        current_user_id,
                    )
                    .execute(&mut *transaction)
                    .await
                    .unwrap()
                    .rows_affected();
                    if rows > 0 {
                        sqlx::query!(
                            "delete from reservations where id = ? and user = ?",
                            path_params.id,
                            reservation.user
                        )
                        .execute(&mut *transaction)
                        .await
                        .unwrap();
                    } else {
                        info!("no matching item found {}", reservation.item);
                    }
                    transaction.commit().await.unwrap();
                    Ok(ReservationsIdAcceptPostResponse::Status200(
                        rows.to_string(),
                    ))
                } else {
                    Ok(ReservationsIdAcceptPostResponse::Status401(
                        "failed to begin transaction".to_string(),
                    ))
                }
            } else {
                Ok(ReservationsIdAcceptPostResponse::Status401(
                    "no reservtion with matching id found".to_string(),
                ))
            }
        } else {
            Ok(ReservationsIdAcceptPostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsIdDeclinePost - POST /api/reservations/{id}/decline"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_decline_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsIdDeclinePostHeaderParams,
        path_params: models::ReservationsIdDeclinePostPathParams,
    ) -> Result<ReservationsIdDeclinePostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows = sqlx::query!("delete from reservations where id = ?", path_params.id)
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();
            Ok(ReservationsIdDeclinePostResponse::Status200(
                rows.to_string(),
            ))
        } else {
            Ok(ReservationsIdDeclinePostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsIdDelete - DELETE /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsIdDeleteHeaderParams,
        path_params: models::ReservationsIdDeletePathParams,
    ) -> Result<ReservationsIdDeleteResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows = sqlx::query!("delete from reservations where id = ?", path_params.id)
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();
            Ok(ReservationsIdDeleteResponse::Status200(rows.to_string()))
        } else {
            Ok(ReservationsIdDeleteResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsIdGet - GET /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdGetPathParams,
    ) -> Result<ReservationsIdGetResponse, String> {
        let item = sqlx::query!(
            r#"
        select
            reservations.id, item, message, reservations.created, name, avatar, email
        from
            reservations
        left join
            users
        on
            reservations.user = users.id
        where
            reservations.id = ?"#,
            path_params.id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();
        if let Some(item) = item {
            let reservation = Reservation {
                id: Some(item.id.to_string()),
                item: Some(item.item.to_string()),
                message: Some(item.message),
                user_name: item.name,
                user_avatar: item.avatar,
                user_email: item.email,
                created: Some(item.created.and_utc()),
            };
            Ok(ReservationsIdGetResponse::Status200(reservation))
        } else {
            Ok(ReservationsIdGetResponse::Status200(Reservation::new()))
        }
    }

    #[doc = r" ReservationsIdPost - POST /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsIdPostHeaderParams,
        path_params: models::ReservationsIdPostPathParams,
        body: models::Reservation,
    ) -> Result<ReservationsIdPostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows = sqlx::query!(
                "update reservations set message = ? where id = ?",
                body.message,
                path_params.id
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

            Ok(ReservationsIdPostResponse::Status200(rows.to_string()))
        } else {
            Ok(ReservationsIdPostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsIdReturnPost - POST /api/reservations/{id}/return"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_return_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsIdReturnPostHeaderParams,
        path_params: models::ReservationsIdReturnPostPathParams,
    ) -> Result<ReservationsIdReturnPostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows = sqlx::query!(
                "update items set status = NULL, reserved = NULL where id = ? and user = ?",
                path_params.id,
                current_user_id
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

            Ok(ReservationsIdReturnPostResponse::Status200(
                rows.to_string(),
            ))
        } else {
            Ok(ReservationsIdReturnPostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ReservationsPut - PUT /api/reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::ReservationsPutHeaderParams,
        body: models::Reservation,
    ) -> Result<ReservationsPutResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let item_id = if let Some(existing_reservation_id) = sqlx::query_scalar!(
                r#"select
                    id
                from
                    reservations
                where
                    user = ? and item = ?"#,
                current_user_id,
                body.item
            )
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            {
                sqlx::query_scalar!(
                    "update reservations set message = ? where id = ?",
                    body.message,
                    existing_reservation_id
                )
                .execute(&self.pool)
                .await
                .unwrap()
                .last_insert_id();
                existing_reservation_id
            } else {
                sqlx::query_scalar!(
                    "insert into reservations (item, user, message) VALUES (?, ?, ?)",
                    body.item,
                    current_user_id,
                    body.message,
                )
                .execute(&self.pool)
                .await
                .unwrap()
                .last_insert_id()
                .try_into()
                .unwrap()
            };
            Ok(ReservationsPutResponse::Status200(item_id.to_string()))
        } else {
            Ok(ReservationsPutResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" SearchGet - GET /api/search"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn search_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::SearchGetQueryParams,
    ) -> Result<SearchGetResponse, String> {
        let search_result = self.search_engine.search(&query_params.text, 20, 0);
        let out = futures::future::join_all(search_result.items.iter().map(|id| async move {
            sqlx::query_as!(
                DbItem,
                r#"
    SELECT
        id,
        title,
        description,
        created,
        updated,
        price_type,
        price,
        location "location: Point",
        place_description,
        category,
        subcategory,
        image,
        user,
        reserved,
        status
    FROM
        items where id = ?
    order by id desc
            "#,
                id
            )
            .fetch_all(&self.pool)
            .await
            .unwrap()
        }))
        .await;
        Ok(openapi::SearchGetResponse::Status200(
            openapi::models::Items::new(
                out.iter()
                    .flatten()
                    .map(|rec| {
                        let mut item = Item::new();
                        item.id = Some(rec.id.unwrap().to_string());
                        item.title.clone_from(&rec.title);
                        item.description.clone_from(&rec.description);

                        if let Some(native_date_time) = rec.created {
                            item.created = Some(native_date_time.and_utc());
                        }
                        if let Some(native_date_time) = rec.updated {
                            item.updated = Some(native_date_time.and_utc());
                        }

                        item.price_type.clone_from(&rec.price_type);
                        item.price = rec.price;

                        if let Some(coordinates) = &rec.location {
                            item.place = Some(ItemPlace {
                                lat: Some(coordinates.lat),
                                lng: Some(coordinates.lng),
                                description: rec.place_description.clone(),
                            })
                        }
                        item.category.clone_from(&rec.category);
                        item.subcategory.clone_from(&rec.subcategory);

                        item.user.clone_from(&rec.user);
                        item.reserved.clone_from(&rec.reserved);

                        item.status.clone_from(&rec.status);
                        item
                    })
                    .collect(),
            ),
        ))
    }

    #[doc = r" UsersGet - GET /api/users"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::UsersGetQueryParams,
    ) -> Result<UsersGetResponse, String> {
        let db_users = sqlx::query_as!(
            DbUser,
            "select id, name, about, avatar, email, phone, joined, last_login from users"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let users = db_users
            .into_iter()
            .map(|db_user| self.db_to_rest_user(db_user))
            .collect::<Vec<User>>();
        Ok(UsersGetResponse::Status200(Users {
            items: users,
            last_evaluated_key: None,
        }))
    }

    #[doc = r" UsersIdDelete - DELETE /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::UsersIdDeleteHeaderParams,
        path_params: models::UsersIdDeletePathParams,
    ) -> Result<UsersIdDeleteResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows_affected = sqlx::query!(
                "delete from users where id = ?",
                path_params.id.parse::<u64>().unwrap()
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();
            Ok(UsersIdDeleteResponse::Status200(rows_affected.to_string()))
        } else {
            Ok(UsersIdDeleteResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" UsersIdGet - GET /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersIdGetPathParams,
    ) -> Result<UsersIdGetResponse, String> {
        let db_user = sqlx::query_as!(DbUser, "select * from users where id = ?", path_params.id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();
        if let Some(db_user) = db_user {
            let user = self.db_to_rest_user(db_user);
            Ok(UsersIdGetResponse::Status200(user))
        } else {
            Ok(UsersIdGetResponse::Status200(User::new()))
        }
    }

    #[doc = r" UsersIdPost - POST /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::UsersIdPostHeaderParams,
        path_params: models::UsersIdPostPathParams,
        body: models::User,
    ) -> Result<UsersIdPostResponse, String> {
        if let Some(current_user_id) = self
            .get_user_id_from_session_fallback_to_token(&cookies, &header_params.authorization)
            .await
        {
            let rows_affected = sqlx::query!(
                "update users set phone = ? where id = ?",
                body.phone,
                current_user_id
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();
            Ok(UsersIdPostResponse::Status200(rows_affected.to_string()))
        } else {
            Ok(UsersIdPostResponse::Status401(
                "no session found".to_string(),
            ))
        }
    }

    #[doc = r" ItemsIdReservationsGet - GET /api/items/{id}/reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_reservations_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdReservationsGetPathParams,
    ) -> Result<ItemsIdReservationsGetResponse, String> {
        let items = sqlx::query!("select reservations.id, item, message, created, name, avatar, email from reservations left join users on user = users.id where item = ?", path_params.id).fetch_all(&self.pool).await.unwrap();
        let items = items
            .into_iter()
            .map(|item| Reservation {
                id: Some(item.id.to_string()),
                item: Some(item.item.to_string()),
                message: Some(item.message),
                user_name: item.name,
                user_avatar: item.avatar,
                user_email: item.email,
                created: Some(item.created.and_utc()),
            })
            .collect::<Vec<Reservation>>();
        Ok(ItemsIdReservationsGetResponse::Status200(
            Reservations::new(items),
        ))
    }
}
