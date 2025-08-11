use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{
    client::{
        create as create_client, delete as delete_client, get_all as get_all_clients,
        get_one as get_one_client, update as update_client,
    },
    collection::{
        create as create_collection, delete as delete_collection, get_all as get_all_collections,
        get_one as get_one_collection, update as update_collection,
    },
    file::{
        delete as delete_file, get_all_for_request, get_one as get_one_file, upload as upload_file,
    },
    firm::{
        create as create_firm, delete as delete_firm, get_all as get_all_firms,
        get_one as get_one_firm, update as update_firm,
    },
    request::{
        create as create_request, delete as delete_request, get_all as get_all_requests,
        get_one as get_one_request, update as update_request,
    },
    user::{
        create as create_user, delete as delete_user, get_all as get_all_users,
        get_one as get_one_user, update as update_user,
    },
};

use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router {
    let clients_router = Router::new()
        .route("/", post(create_client).get(get_all_clients))
        .route(
            "/:id",
            get(get_one_client)
                .patch(update_client)
                .delete(delete_client),
        );

    let users_router = Router::new()
        .route("/", post(create_user).get(get_all_users))
        .route(
            "/:id",
            get(get_one_user).patch(update_user).delete(delete_user),
        );

    let firms_router = Router::new()
        .route("/", post(create_firm).get(get_all_firms))
        .route(
            "/:id",
            get(get_one_firm).patch(update_firm).delete(delete_firm),
        );

    // Note: File routes are different. Upload is on a request, get_all is also on a request.
    let files_router = Router::new().route("/:id", get(get_one_file).delete(delete_file));

    let requests_router = Router::new()
        .route("/", post(create_request).get(get_all_requests))
        .route(
            "/:id",
            get(get_one_request)
                .patch(update_request)
                .delete(delete_request),
        )
        .route(
            "/:request_id/files",
            get(get_all_for_request).post(upload_file),
        );

    let collections_router = Router::new()
        .route("/", post(create_collection).get(get_all_collections))
        .route(
            "/:id",
            get(get_one_collection)
                .patch(update_collection)
                .delete(delete_collection),
        );

    Router::new()
        .nest("/clients", clients_router)
        .nest("/users", users_router)
        .nest("/firms", firms_router)
        .nest("/files", files_router)
        .nest("/requests", requests_router)
        .nest("/collections", collections_router)
        .with_state(pool)
}

