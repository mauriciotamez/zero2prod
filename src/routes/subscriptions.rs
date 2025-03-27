use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new suscriber.",
        %request_id,
        suscriber.email = %form.email,
        suscriber.name = %form.name
    );

    let _request_span_guard = request_span.enter();
    tracing::info!(
        "Request ID '{}' - Adding '{}' '{}' as a new subscription",
        request_id,
        form.email,
        form.name
    );

    let query_span = tracing::info_span!("Saving new subscription to database",);
    tracing::info!("Request ID '{}' - Saving new subscription.", request_id);
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Request ID '{}' - New subscribe for {}",
                request_id,
                form.email
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "Request ID '{}' - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
