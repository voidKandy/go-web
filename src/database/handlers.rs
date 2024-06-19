use super::{error::DatabaseResult, model::*};
use sqlx::{Pool, Postgres};
use tracing::{info, instrument};
use uuid::Uuid;

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: Uuid) -> DatabaseResult<Option<User>> {
    info!("Getting user by id: {:?}", id);
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(pool: &Pool<Postgres>, email: &str) -> DatabaseResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email.to_ascii_lowercase())
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

/// Takes hashed password
pub async fn register_new_user(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    subscribed: bool,
    hashed_password: &str,
    admin: bool,
) -> DatabaseResult<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name,email,password,subscribed,admin) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(name.to_string())
    .bind(email.to_string().to_ascii_lowercase())
    .bind(hashed_password.to_string())
    .bind(subscribed)
    .bind(admin)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

#[tracing::instrument(name = "Get post by title", skip(pool))]
pub async fn get_post_by_title(pool: &Pool<Postgres>, title: &str) -> DatabaseResult<Option<Post>> {
    let user = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE title = $1")
        .bind(title)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

#[instrument(name = "get recent post")]
pub async fn get_most_recent_posts(
    pool: &Pool<Postgres>,
    category_opt: Option<String>,
) -> DatabaseResult<Vec<Post>> {
    let query = match category_opt {
        Some(cat) => sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE category = $1 ORDER BY created_at DESC LIMIT 5",
        )
        .bind(cat),
        None => sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC LIMIT 5"),
    };
    let post = query.fetch_all(pool).await?;
    Ok(post)
}

#[instrument(name = "delete post", skip(pool))]
pub async fn delete_post_by_title(pool: &Pool<Postgres>, title: &str) -> DatabaseResult<()> {
    sqlx::query("DELETE FROM posts WHERE title = $1")
        .bind(title)
        .execute(pool)
        .await?;
    Ok(())
}

#[tracing::instrument(name = "post blog post", skip(pool))]
pub async fn post_blog_post(pool: &Pool<Postgres>, post: UploadBlogPost) -> DatabaseResult<Post> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, subtitle, category, content) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&post.title)
    .bind(&post.subtitle)
    .bind(&post.category)
    .bind(&post.content)
    .fetch_one(pool)
    .await?;
    info!("blog post posted to database successfully");
    Ok(post)
}

#[tracing::instrument(name = "patch post", skip(pool))]
pub async fn patch_blog_post(
    pool: &Pool<Postgres>,
    title: &str,
    post: UpdateBlogPost,
) -> DatabaseResult<Option<Post>> {
    // First, fetch the post by its title
    let id: Uuid = sqlx::query_scalar("SELECT id FROM posts WHERE title = $1")
        .bind(&post.title)
        .fetch_one(pool)
        .await?;
    info!("got id: {:?}", id);
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(title) = post.title {
        updates.push("title = $2");
        params.push(title);
    }

    if let Some(subtitle) = post.subtitle {
        updates.push("subtitle = $3");
        params.push(subtitle);
    }

    if let Some(category) = post.category {
        updates.push("category = $4");
        params.push(category);
    }

    if let Some(content) = post.content {
        updates.push("content = $5");
        params.push(content);
    }

    if updates.is_empty() {
        return Ok(None); // No fields to update, return existing post
    }

    let set_clause = updates.join(", ");
    let query = format!("UPDATE posts SET {} WHERE id = $1 RETURNING *", set_clause);
    info!("update handler built query: {}", query);

    let mut query_builder = sqlx::query_as::<_, Post>(&query).bind(id);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    let updated_post = query_builder.fetch_one(pool).await?;

    Ok(Some(updated_post))
}

pub async fn get_blog_categories(pool: &Pool<Postgres>) -> DatabaseResult<Vec<String>> {
    info!("getting categories");
    let categories = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT category FROM posts WHERE category is NOT NULL",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(category,)| category)
    .collect();
    Ok(categories)
}
