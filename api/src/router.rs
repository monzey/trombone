use axum::{
    routing::{delete, get, patch, post},
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
        create as create_file, delete as delete_file, get_all as get_all_files,
        get_one as get_one_file, update as update_file,
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
        .route("/", post(create_client))
        .route("/", get(get_all_clients))
        .route("/:id", get(get_one_client))
        .route("/:id", patch(update_client))
        .route("/:id", delete(delete_client));

    let users_router = Router::new()
        .route("/", post(create_user))
        .route("/", get(get_all_users))
        .route("/:id", get(get_one_user))
        .route("/:id", patch(update_user))
        .route("/:id", delete(delete_user));

    let firms_router = Router::new()
        .route("/", post(create_firm))
        .route("/", get(get_all_firms))
        .route("/:id", get(get_one_firm))
        .route("/:id", patch(update_firm))
        .route("/:id", delete(delete_firm));

    let files_router = Router::new()
        .route("/", post(create_file))
        .route("/", get(get_all_files))
        .route("/:id", get(get_one_file))
        .route("/:id", patch(update_file))
        .route("/:id", delete(delete_file));

    let requests_router = Router::new()
        .route("/", post(create_request))
        .route("/", get(get_all_requests))
        .route("/:id", get(get_one_request))
        .route("/:id", patch(update_request))
        .route("/:id", delete(delete_request));

    let collections_router = Router::new()
        .route("/", post(create_collection))
        .route("/", get(get_all_collections))
        .route("/:id", get(get_one_collection))
        .route("/:id", patch(update_collection))
        .route("/:id", delete(delete_collection));

    Router::new()
        .nest("/clients", clients_router)
        .nest("/users", users_router)
        .nest("/firms", firms_router)
        .nest("/files", files_router)
        .nest("/requests", requests_router)
        .nest("/collections", collections_router)
        .with_state(pool)
}
