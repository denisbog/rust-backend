use std::u64;

use async_trait::async_trait;
use openapi::ItemsContentGetResponse;
use openapi::ItemsContentNameDeleteResponse;
use openapi::ItemsContentNameGetResponse;
use openapi::ItemsContentNamePutResponse;
use openapi::ItemsIdContentNameDeleteResponse;

use crate::ServerImpl;
use anyhow::Context;
use async_session::{Session, SessionStore};
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use common::DbItem;
use common::DbUser;
use common::Point;
use http::Method;
use oauth2::TokenResponse;
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope};
use openapi::models::Item;
use openapi::models::ItemPlace;
use openapi::models::Reservation;
use openapi::models::Reservations;
use openapi::models::Users;
use openapi::models::{self, User};
use openapi::types::ByteArray;
use openapi::ItemsIdContentNameGetResponse;
use openapi::ItemsIdContentNamePutResponse;
use openapi::SearchGetResponse;
use openapi::{
    AuthorizedGetResponse, ItemsGetResponse, ItemsIdDeleteResponse, ItemsIdGetResponse,
    ItemsIdPostResponse, ItemsPutResponse, LoginGetResponse, MyItemsGetResponse,
    MyRelatedGetResponse, ReservationsGetResponse, ReservationsIdAcceptPostResponse,
    ReservationsIdDeclinePostResponse, ReservationsIdDeleteResponse, ReservationsIdGetResponse,
    ReservationsIdPostResponse, ReservationsIdReturnPostResponse, ReservationsPutResponse,
    UsersGetResponse, UsersIdDeleteResponse, UsersIdGetResponse, UsersIdPostResponse,
};

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
                .context("failed in sending request request to authorization server")
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

        let client = reqwest::Client::new();
        let user_data: User = client
            .get("https://graph.facebook.com/v19.0/me?fields=name,first_name,last_name,email")
            .bearer_auth(token)
            .send()
            .await
            .context("failed in sending request to target Url")
            .unwrap()
            .json::<User>()
            .await
            .context("failed to deserialize response as JSON")
            .unwrap();

        let mut session = Session::new();
        session.insert("user", &user_data).unwrap();

        self.check_and_import_user_if_required(&user_data).await;

        // Store session and get corresponding cookie
        let cookie = &self.store.store_session(session).await.unwrap().unwrap();

        Ok(openapi::AuthorizedGetResponse::Status302 {
            location: Some("/api/items".into()),
            set_cookie: Some(format!("session={cookie}; SameSite=Lax; Path=/")),
        })
    }

    #[doc = r" ItemsContentGet - GET /api/items/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_content_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<ItemsContentGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            if let Some(contents) = self.get_images_for_item(&current_user_id).await {
                Ok(ItemsContentGetResponse::Status200(contents))
            } else {
                Err("no content found".to_string())
            }
        } else {
            Err("no session found".to_string())
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
        path_params: models::ItemsContentNameDeletePathParams,
    ) -> Result<ItemsContentNameDeleteResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            self.delete_content(&current_user_id, &path_params.name)
                .await
                .unwrap();
            Ok(ItemsContentNameDeleteResponse::Status200("ok".to_string()))
        } else {
            Err("no session found".to_string())
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
        path_params: models::ItemsContentNameGetPathParams,
    ) -> Result<ItemsContentNameGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            if let Some(contents) = self.get_content(&current_user_id, &path_params.name).await {
                Ok(ItemsContentNameGetResponse::Status200(ByteArray(contents)))
            } else {
                Err("file not found".to_string())
            }
        } else {
            Err("no session found".to_string())
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
        path_params: models::ItemsContentNamePutPathParams,
        body: Bytes,
    ) -> Result<ItemsContentNamePutResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            self.upload_content(&current_user_id, &path_params.name, &body)
                .await;
            Ok(ItemsContentNamePutResponse::Status200("ok".to_string()))
        } else {
            Ok(ItemsContentNamePutResponse::Status200(
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
        if let Some(current_user_id) = cookies.get("session") {
            self.get_session_user_id(&cookies).await;
        }
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
        path_params: models::ItemsIdContentNameDeletePathParams,
    ) -> Result<ItemsIdContentNameDeleteResponse, String> {
        self.delete_content(&path_params.id, &path_params.name)
            .await
            .unwrap();
        Ok(ItemsIdContentNameDeleteResponse::Status200(
            "ok".to_string(),
        ))
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
            Err("file not found".to_string())
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
        path_params: models::ItemsIdContentNamePutPathParams,
        body: Bytes,
    ) -> Result<ItemsIdContentNamePutResponse, String> {
        self.upload_content(&path_params.id, &path_params.name, &body)
            .await;
        Ok(ItemsIdContentNamePutResponse::Status200("ok".to_string()))
    }

    #[doc = r" ItemsIdDelete - DELETE /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdDeletePathParams,
    ) -> Result<ItemsIdDeleteResponse, String> {
        let rows = sqlx::query!("delete from items where id = ?", path_params.id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

        self.search_engine
            .delete(path_params.id.parse::<u64>().unwrap())
            .await;

        Ok(ItemsIdDeleteResponse::Status200(rows.to_string()))
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
        path_params: models::ItemsIdPostPathParams,
        body: models::Item,
    ) -> Result<ItemsIdPostResponse, String> {
        let session = cookies.get("session").unwrap();
        let current_user_id = self.get_session_user_id(&cookies).await;

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
            body.status,
            path_params.id
        )
        .execute(&self.pool)
        .await
        .unwrap()
        .rows_affected();

        self.search_engine
            .index(&DbItem {
                id: Some(body.id.unwrap().parse::<u64>().unwrap()),
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
    }

    #[doc = r" ItemsPut - PUT /api/items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Item,
    ) -> Result<ItemsPutResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
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
                Ok(ItemsPutResponse::Status200(format!(
                    "failed to get transaction"
                )))
            }
        } else {
            Err("no session found".to_string())
        }
    }

    #[doc = r" LoginGet - GET /api/login"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn login_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<LoginGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            tracing::info!("user session found {current_user_id}");
            Ok(openapi::LoginGetResponse::Status302 {
                location: Some("/api/items".into()),
            })
        } else {
            let (auth_url, _csrf_token) = self
                .oauth_client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("email".to_string()))
                .url();

            Ok(openapi::LoginGetResponse::Status302 {
                location: Some(auth_url.to_string()),
            })
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
        query_params: models::MyItemsGetQueryParams,
    ) -> Result<MyItemsGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
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
            Ok(openapi::MyItemsGetResponse::Status200(
                openapi::models::Items::new(Vec::new()),
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
        query_params: models::MyRelatedGetQueryParams,
    ) -> Result<MyRelatedGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
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
            Ok(openapi::MyRelatedGetResponse::Status200(
                openapi::models::Items::new(Vec::new()),
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
        query_params: models::ReservationsGetQueryParams,
    ) -> Result<ReservationsGetResponse, String> {
        let items = sqlx::query!("select reservations.id, item, message, created, name, avatar, email from reservations left join users on user = users.id").fetch_all(&self.pool).await.unwrap();
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
    }

    #[doc = r" ReservationsIdAcceptPost - POST /api/reservations/{id}/accept"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_accept_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdAcceptPostPathParams,
    ) -> Result<ReservationsIdAcceptPostResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            let reservation = sqlx::query!(
                "select user, item from reservations where id = ?",
                path_params.id
            )
            .fetch_optional(&self.pool)
            .await
            .unwrap();
            if let Some(reservation) = reservation {
                if let Ok(mut transaction) = self.pool.begin().await {
                    sqlx::query!("delete from reservations where id = ?", path_params.id)
                        .execute(&mut *transaction)
                        .await
                        .unwrap();
                    let rows = sqlx::query!(
                        "update items set reserved = ?, status = ? where id = ?",
                        reservation.user,
                        "rented",
                        reservation.item,
                    )
                    .execute(&mut *transaction)
                    .await
                    .unwrap()
                    .rows_affected();
                    transaction.commit().await.unwrap();
                    Ok(ReservationsIdAcceptPostResponse::Status200(
                        rows.to_string(),
                    ))
                } else {
                    Ok(ReservationsIdAcceptPostResponse::Status200(
                        "failed to begin transaction".to_string(),
                    ))
                }
            } else {
                Ok(ReservationsIdAcceptPostResponse::Status200(
                    "no reservtion with matching id found".to_string(),
                ))
            }
        } else {
            Ok(ReservationsIdAcceptPostResponse::Status200(
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
        path_params: models::ReservationsIdDeclinePostPathParams,
    ) -> Result<ReservationsIdDeclinePostResponse, String> {
        let rows = sqlx::query!("delete from reservations where id = ?", path_params.id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();
        Ok(ReservationsIdDeclinePostResponse::Status200(
            rows.to_string(),
        ))
    }

    #[doc = r" ReservationsIdDelete - DELETE /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdDeletePathParams,
    ) -> Result<ReservationsIdDeleteResponse, String> {
        let rows = sqlx::query!("delete from reservations where id = ?", path_params.id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();
        Ok(ReservationsIdDeleteResponse::Status200(rows.to_string()))
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
        path_params: models::ReservationsIdPostPathParams,
        body: models::Reservation,
    ) -> Result<ReservationsIdPostResponse, String> {
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
    }

    #[doc = r" ReservationsIdReturnPost - POST /api/reservations/{id}/return"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_return_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdReturnPostPathParams,
    ) -> Result<ReservationsIdReturnPostResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            let rows = sqlx::query!(
                "update items set status = NULL, reserved = NULL where id = ? and reserved = ?",
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
            Ok(ReservationsIdReturnPostResponse::Status200(
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
        body: models::Reservation,
    ) -> Result<ReservationsPutResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            let item_id = sqlx::query_scalar!(
                "insert into reservations (item, user, message) VALUES (?, ?, ?)",
                body.item,
                current_user_id,
                body.message,
            )
            .execute(&self.pool)
            .await
            .unwrap()
            .last_insert_id();

            Ok(ReservationsPutResponse::Status200(item_id.to_string()))
        } else {
            Ok(ReservationsPutResponse::Status200(
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
                        item.title = rec.title.clone();
                        item.description = rec.description.clone();

                        if let Some(native_date_time) = rec.created {
                            item.created = Some(native_date_time.and_utc());
                        }
                        if let Some(native_date_time) = rec.updated {
                            item.updated = Some(native_date_time.and_utc());
                        }

                        item.price_type = rec.price_type.clone();
                        item.price = rec.price;

                        if let Some(coordinates) = &rec.location {
                            item.place = Some(ItemPlace {
                                lat: Some(coordinates.lat),
                                lng: Some(coordinates.lng),
                                description: rec.place_description.clone(),
                            })
                        }
                        item.category = rec.category.clone();
                        item.subcategory = rec.subcategory.clone();

                        item.user = rec.user.clone();
                        item.reserved = rec.reserved.clone();

                        item.status = rec.status.clone();
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
            "select id, name, about, avatar, email, joined, last_login from users"
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
        path_params: models::UsersIdDeletePathParams,
    ) -> Result<UsersIdDeleteResponse, String> {
        let rows_affected = sqlx::query!(
            "delete from users where id = ?",
            path_params.id.parse::<u64>().unwrap()
        )
        .execute(&self.pool)
        .await
        .unwrap()
        .rows_affected();

        Ok(UsersIdDeleteResponse::Status200(rows_affected.to_string()))
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
            .fetch_one(&self.pool)
            .await
            .unwrap();
        let user = self.db_to_rest_user(db_user);
        Ok(UsersIdGetResponse::Status200(user))
    }

    #[doc = r" UsersIdPost - POST /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersIdPostPathParams,
    ) -> Result<UsersIdPostResponse, String> {
        todo!()
    }
}
