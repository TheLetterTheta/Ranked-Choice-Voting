use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error, get,
    middleware::Logger,
    post,
    web::{self, Json, ServiceConfig},
    Result,
};
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool, Row};
use std::ops::Add;
mod db_types;
mod dto;

#[post("create_poll")]
async fn create_poll(
    poll: web::Form<dto::CreatePoll>,
    state: web::Data<AppState>,
) -> Result<Json<dto::Poll>> {
    let expiration = format!("{}Z{}", poll.expiration, poll.timezone);
    let expiration =
        DateTime::<FixedOffset>::parse_from_str(&dbg!(expiration), "%Y-%m-%dT%H:%MZ%z");

    let expires_at: DateTime<Utc>;
    let now = Utc::now();

    match (poll.lasts_for, expiration) {
        (None, Err(_)) => {
            return Err(error::ErrorBadRequest(
                "Expiration date or duration must be specified",
            ));
        }
        (_, Ok(expiration)) => {
            expires_at = expiration.into();
        }
        (Some(minutes), Err(_)) => {
            expires_at = now.add(Duration::minutes(minutes.into()));
        }
    }

    let new_poll = sqlx::query("SELECT rcv.StartPoll(NULL, $1, $2, $3, $4)")
        .bind(&poll.title)
        .bind(&poll.description)
        .bind(now)
        .bind(expires_at)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(web::Json(dto::Poll {
        id: new_poll.get(0),
        title: poll.title.clone(),
        description: poll.description.clone(),
        expiration: expires_at,
        created_at: now,
    }))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PG_PASSWORD}@localhost:{secrets.PG_PORT}/postgres"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!().run(&pool).await.map_err(CustomError::new);

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(state)
                .wrap(Logger::default())
                .service(create_poll),
        )
        .service(
            Files::new("/", "./leptos/dist/.")
                .index_file("index.html")
                .prefer_utf8(true)
                .default_handler(|req: ServiceRequest| {
                    let (http_req, _payload) = req.into_parts();

                    async {
                        let response = actix_files::NamedFile::open("./leptos/dist/index.html")?
                            .into_response(&http_req);
                        Ok(ServiceResponse::new(http_req, response))
                    }
                }),
        );
    };

    Ok(config.into())
}
