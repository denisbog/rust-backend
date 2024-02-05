use async_trait::async_trait;

use axum::extract::Host;
use axum_extra::extract::CookieJar;
use http::Method;

use crate::ServerImpl;
use anyhow::Context;
use async_session::{Session, SessionStore};
use bytes::Bytes;
use common::DbItem;
use common::Point;
use oauth2::TokenResponse;
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope};
use openapi::models::{self, ItemPlace, User};
use openapi::SearchGetResponse;
use openapi::{models::Item, ItemsIdContentPutResponse};
use openapi::{
    AuthorizedGetResponse, ItemsGetResponse, ItemsIdContentGetResponse, ItemsIdDeleteResponse,
    ItemsIdGetResponse, ItemsIdPostResponse, ItemsPutResponse, LoginGetResponse,
    MyItemsGetResponse, MyRelatedGetResponse, ReservationsGetResponse,
    ReservationsIdAcceptPostResponse, ReservationsIdDeclinePostResponse,
    ReservationsIdDeleteResponse, ReservationsIdGetResponse, ReservationsIdPostResponse,
    ReservationsIdReturnPostResponse, ReservationsPutResponse, UsersGetResponse,
    UsersIdDeleteResponse, UsersIdGetResponse, UsersIdPostResponse,
};
#[allow(unused_variables)]
#[async_trait]
impl openapi::Api for ServerImpl {
    #[doc = r" ItemsIdContentGet - GET /api/items/{id}/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdContentGetPathParams,
    ) -> Result<ItemsIdContentGetResponse, String> {
        todo!()
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
        todo!()
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
        todo!()
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
        let recs = sqlx::query!(
            r#"
    SELECT subcategory
    FROM items
            "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(openapi::MyItemsGetResponse::Status200(
            openapi::models::Items::new(
                recs.into_iter()
                    .map(|rec| {
                        let mut item = Item::new();
                        item.subcategory = Some(rec.subcategory);
                        item
                    })
                    .collect(),
            ),
        ))
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
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

    #[doc = r" AuthorizedGet - GET /api/authorized"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn authorized_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::AuthorizedGetQueryParams,
    ) -> Result<AuthorizedGetResponse, String> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(query_params.code))
            .request_async(async_http_client)
            .await
            .context("failed in sending request request to authorization server")
            .unwrap();

        let client = reqwest::Client::new();
        let user_data: User = client
            .get("https://graph.facebook.com/v19.0/me?fields=name,first_name,last_name,email")
            .bearer_auth(token.access_token().secret())
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

        // Store session and get corresponding cookie
        let cookie = &self.store.store_session(session).await.unwrap().unwrap();

        Ok(openapi::AuthorizedGetResponse::Status302 {
            location: Some("/api/items".into()),
            set_cookie: Some(format!("session={cookie}; SameSite=Lax; Path=/")),
        })
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

        let recs = sqlx::query_as!(
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
        user,
        reserved,
        status
    FROM 
        items
    ORDER BY id desc
            "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(openapi::ItemsGetResponse::Status200(
            openapi::models::Items::new(
                recs.into_iter()
                    .map(|rec| {
                        let mut item = Item::new();
                        item.id = Some(rec.id.unwrap().to_string());
                        item.title = rec.title;
                        item.description = rec.description;

                        if let Some(native_date_time) = rec.created {
                            item.created = Some(native_date_time.and_utc());
                        }
                        if let Some(native_date_time) = rec.updated {
                            item.updated = Some(native_date_time.and_utc());
                        }

                        item.price_type = rec.price_type;
                        item.price = rec.price;

                        if let Some(coordinates) = rec.location {
                            item.place = Some(ItemPlace {
                                lat: Some(coordinates.lat),
                                lng: Some(coordinates.lng),
                                description: rec.place_description,
                            })
                        }
                        item.category = rec.category;
                        item.subcategory = rec.subcategory;

                        item.user = rec.user;
                        item.reserved = rec.reserved;

                        item.status = rec.status;
                        item
                    })
                    .collect(),
            ),
        ))
    }

    #[doc = r" ItemsIdContentPut - PUT /api/items/{id}/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdContentPutPathParams,
        body: Bytes,
    ) -> Result<ItemsIdContentPutResponse, String> {
        todo!()
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
        Ok(openapi::ItemsIdGetResponse::Status200(
            openapi::models::Item::new(),
        ))
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
        let session = cookies.get("session").unwrap();
        let current_user_id = self.get_session_user_id(&cookies).await;
        let place = body.place.unwrap();
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
        user,
        reserved,
        status
        )
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?, ?, ?)
            "#,
                body.title,
                body.description,
                body.price_type,
                body.price,
                place.lat,
                place.lng,
                place.description,
                body.category,
                body.subcategory,
                current_user_id,
                body.reserved,
                body.status
            )
            .execute(&mut *transaction)
            .await
            .unwrap()
            .last_insert_id();
            let _ = transaction.commit().await;
            Ok(ItemsPutResponse::Status200(format!("{item_id}")))
        } else {
            Ok(ItemsPutResponse::Status200(format!(
                "failed to get transaction"
            )))
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
}
