use axum_csrf::{CsrfConfig, CsrfToken};
use serde::{Deserialize, Serialize};
#[derive( Deserialize, Serialize)]
struct Keys {
    authenticity_token: String,

}
