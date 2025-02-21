use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: Uuid,
    title: String,
    content: String,
}

#[derive(Clone)]
struct Microblog {
    db_pool: Arc<Pool<Sqlite>>,
}

impl Microblog {
    fn new(database_url: &str) -> Self {
        let pool = Arc::new(
            SqlitePool::connect_lazy(database_url).expect("Failed to connect to database"),
        );
        Microblog { db_pool: pool }
    }

    async fn create_post(&self, title: String, content: String) -> Result<Post, String> {
        let post = Post {
            id: Uuid::new_v4(),
            title,
            content,
        };

        let id_string = post.id.to_string();
        sqlx::query!(
            "INSERT INTO posts (id, title, content) VALUES (?, ?, ?)",
            id_string,
            post.title,
            post.content
        )
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database insert error: {:?}", e);
            "Failed to insert post into database"
        })?;

        Ok(post)
    }

    async fn get_posts(&self) -> Result<Vec<Post>, String> {
        let rows = sqlx::query!("SELECT id, title, content FROM posts")
            .fetch_all(&*self.db_pool)
            .await
            .map_err(|_| "Failed to fetch posts from database")?;

        let posts: Vec<Post> = rows
            .into_iter()
            .map(|row| Post {
                id: Uuid::parse_str(row.id.as_deref().unwrap_or("")).expect("Invalid UUID in DB"),
                title: row.title,
                content: row.content,
            })
            .collect();
        Ok(posts)
    }
}

fn json_response<T: Serialize>(
    data: Result<T, String>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match data {
        Ok(value) => match serde_json::to_string(&value) {
            Ok(response_body) => Ok(Response::new(Full::new(Bytes::from(response_body)))),
            Err(e) => {
                error!("Failed to read request body: {:?}", e);
                Ok(error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read request body",
                ))
            }
        },
        Err(e) => {
            error!("Failed to read request body: {:?}", e);
            Ok(error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read request body",
            ))
        }
    }
}

async fn create_post(
    req: Request<hyper::body::Incoming>,
    blog: Arc<Microblog>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("Create post");
    let whole_body = match req.collect().await {
        Ok(body) => body.to_bytes(),
        Err(e) => {
            error!("Failed to read request body: {:?}", e);
            return Ok(error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read request body",
            ));
        }
    };

    let post_data: serde_json::Value = match serde_json::from_slice(&whole_body) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to read request body: {:?}", e);
            return Ok(error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read request body",
            ));
        }
    };

    let title = post_data["title"].as_str().unwrap_or("").to_string();
    let content = post_data["content"].as_str().unwrap_or("").to_string();

    if title.is_empty() || content.is_empty() {
        return Ok(error_response(
            StatusCode::BAD_REQUEST,
            "Title and content cannot be empty",
        ));
    }

    json_response(blog.create_post(title, content).await)
}

async fn get_posts(blog: Arc<Microblog>) -> Result<Response<Full<Bytes>>, Infallible> {
    json_response(blog.get_posts().await)
}

fn error_response(status: StatusCode, message: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(message.to_string())))
        .unwrap_or_else(|_| Response::new(Full::new(Bytes::from("Internal Server Error"))))
}

async fn router(
    req: Request<hyper::body::Incoming>,
    blog: Arc<Microblog>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/posts") => create_post(req, blog).await,
        (&Method::GET, "/posts") => get_posts(blog).await,
        _ => Ok(error_response(StatusCode::NOT_FOUND, "Not Found")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let blog = Arc::new(Microblog::new("sqlite://microblog.db"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Server running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let blog_clone = Arc::clone(&blog);

        tokio::task::spawn(async move {
            let io = TokioIo::new(stream);

            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| router(req, Arc::clone(&blog_clone))),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
