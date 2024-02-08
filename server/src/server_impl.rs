use std::u64;

use async_trait::async_trait;

use axum::extract::Host;
use axum_extra::extract::CookieJar;
use common::DbUser;
use http::Method;
use openapi::models::ItemPlace;
use openapi::models::Users;
use sqlx::FromRow;
use sqlx::MySql;
use sqlx::QueryBuilder;

use crate::ServerImpl;
use anyhow::Context;
use async_session::{Session, SessionStore};
use bytes::Bytes;
use common::DbItem;
use common::Point;
use oauth2::TokenResponse;
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope};
use openapi::models::{self, User};
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

use sqlx::Row;

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
        let db_users = sqlx::query_as!(DbUser, "select * from users")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let users = db_users
            .into_iter()
            .map(|db_user| User {
                id: db_user.id,
                name: db_user.name,
                about: db_user.about,
                avatar: db_user.avatar,
                email: db_user.email,
                joined: if let Some(native_date_time) = db_user.joined {
                    Some(native_date_time.and_utc())
                } else {
                    None
                },
                last_login: if let Some(native_date_time) = db_user.joined {
                    Some(native_date_time.and_utc())
                } else {
                    None
                },
            })
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
        let db_user = sqlx::query_as!(DbUser, "select * from users where id = ?", path_params.id)
            .fetch_one(&self.pool)
            .await
            .unwrap();
        let user = User {
            id: db_user.id,
            name: db_user.name,
            about: db_user.about,
            avatar: db_user.avatar,
            email: db_user.email,
            joined: if let Some(native_date_time) = db_user.joined {
                Some(native_date_time.and_utc())
            } else {
                None
            },
            last_login: if let Some(native_date_time) = db_user.joined {
                Some(native_date_time.and_utc())
            } else {
                None
            },
        };
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

        self.check_and_import_user_if_required(&user_data).await;

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

        let query = r#"
    SELECT
        items.id,
        items.title,
        items.description,
        items.created,
        items.updated,
        items.price_type,
        items.price,
        items.location,
        items.place_description,
        items.category,
        items.subcategory,
        items.user,
        items.reserved,
        items.status,
        users.name,
        users.email,
        users.avatar,
        users.joined,
        users.last_login
    FROM 
        items
    LEFT JOIN
        users
    ON
        items.user = users.id"#;

        let mut builder = QueryBuilder::<MySql>::new(query);

        let mut append_condition = false;

        if let Some(ref category) = query_params.category {
            if !append_condition {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" category = ");
            builder.push_bind(category);
            if let Some(ref subcategory) = query_params.subcategory {
                builder.push(" and subcategory in (");
                let mut separated = builder.separated(',');
                subcategory.split(',').into_iter().for_each(|subcategory| {
                    separated.push_bind(subcategory);
                });
                separated.push_unseparated(')');
            }
        }

        if let (Some(lat), Some(lng), Some(r)) =
            (query_params.lat, query_params.long, query_params.r)
        {
            if !append_condition {
                builder.push(" WHERE");
                append_condition = true;
            } else {
                builder.push(" and");
            }
            builder.push(" ST_DISTANCE_SPHERE(location, POINT(");
            builder.push_bind(lat);
            builder.push(", ");
            builder.push_bind(lng);
            builder.push(")) < ");
            builder.push_bind(r);
        }

        if let Some(last_id) = query_params.last_evaluated_key {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
            }
            builder.push(" items.id <= ");
            builder.push_bind(last_id);
        }
        builder.push(" ORDER BY id DESC");
        builder.push(" LIMIT 11");
        // println!("{}", builder.sql());
        let execute_query = builder.build();
        let recs = execute_query.fetch_all(&self.pool).await.unwrap();

        let last_evaluated_key: Option<String> = if recs.len() > 10 {
            let last_id: Option<u64> = recs.last().unwrap().try_get("id").unwrap();
            Some(last_id.unwrap().to_string())
        } else {
            None
        };

        Ok(openapi::ItemsGetResponse::Status200(
            openapi::models::Items {
                items: recs
                    .into_iter()
                    .take(10)
                    .map(|row| {
                        let rec = DbItem::from_row(&row).unwrap();
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

                        item.user_name = row.try_get("name").unwrap();
                        item.user_email = row.try_get("email").unwrap();
                        item.user_avatar = row.try_get("avatar").unwrap();

                        item
                    })
                    .collect(),
                last_evaluated_key,
            },
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
