//! Rocket handler to serve static content as FileServer in Rocket does not support authentication

use crate::oidc::guard::OidcUser;

use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

/// Handler to serve static content
///
/// # Arguments
/// * `path` - path of front-end routes to be routed to the SPA frontend. Note: Rocket automatically makes sure that they do not allow path-traversal
/// * `user` - provided by Rocket only if user is authenticated => otherwise handler cannot be accessed
/// 
/// Returns the content of the file
///
#[get("/<path..>",rank=2)]
pub async fn serve_static(path: PathBuf, user: OidcUser) -> Option<NamedFile> {
    let mut path = Path::new("./static").join(path);
    if path.is_dir() { 
        path = path.join("index.html");
    }
    NamedFile::open(path).await.ok()
}
