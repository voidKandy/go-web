use crate::{
    database::model::{Attachment, UpdateBlogPost, UploadBlogPost},
    routing::error::{RouterError, RouterResult},
};
use anyhow::anyhow;
use axum::extract::Multipart;
use std::fs;
use std::path::Path;
use std::{fs::File, io::Write, ops::Deref};
use tracing::{error, info, warn};

const POSTS_DIRECTORY_PATH: &str = "/public/assets/posts";

/// saves all attachments and returns the content of the markdown file
#[tracing::instrument(name = "save all attachments")]
pub(super) fn save_all_attachments_to_filesystem(
    mut vec: Vec<Attachment>,
) -> anyhow::Result<Option<String>> {
    let path_str = format!("./{}", &POSTS_DIRECTORY_PATH);

    let parent_path_str = path_str.rsplit_once('/').unwrap().0;
    let public_path_str = parent_path_str.rsplit_once('/').unwrap().0;

    let public_metadata = fs::metadata(public_path_str)?;
    let mut public_perms = public_metadata.permissions();
    public_perms.set_readonly(false);

    let parent_metadata = fs::metadata(parent_path_str)?;
    let mut parent_perms = parent_metadata.permissions();
    parent_perms.set_readonly(false);

    let path = Path::new(&path_str);
    if !path.exists() {
        fs::create_dir(path).map_err(|err| {
            error!(
                "there s an error when creating the posts assets directory: {:?}",
                err
            );
            anyhow!(
                "there was an error when creating the posts assets directory: {:?}",
                err
            )
        })?;
    }

    let mut ret: Option<String> = None;
    for attachment in vec.iter_mut() {
        if attachment.filename.split_once('.').unwrap().1 == "md" {
            if ret.is_some() {
                warn!("don't pass multiple markdown files to save_all")
            }
            ret = Some(String::from_utf8(attachment.bytes.drain(..).collect())?);
        } else {
            let path_str = format!(".{}/{}", POSTS_DIRECTORY_PATH, attachment.filename);
            let path = Path::new(&path_str);
            match path.exists() {
                false => {
                    warn!("file: {:?} does not exist, writing", path);
                    let mut file = File::create_new(path).map_err(|err| anyhow!(err))?;
                    file.write_all(&attachment.bytes)
                        .map_err(|err| anyhow!(err))?;
                }
                true => {
                    warn!("file: {:?} already exists, overwriting", path);
                    fs::write(path, &attachment.bytes).map_err(|err| {
                        error!(
                            "there was a problem overriting the file: {:?}\n: {:?}",
                            path, err
                        );
                        anyhow!(
                            "there was a problem overriting the file: {:?}\n: {:?}",
                            path,
                            err
                        )
                    })?;
                }
            }
        }
    }

    public_perms.set_readonly(true);
    parent_perms.set_readonly(true);
    match ret {
        Some(text) => {
            warn!("content returned from save_all: {}", text);
            return Ok(Some(change_attachments_links(&text)?));
        }
        None => {
            warn!("no content returned from save_all");
            return Ok(None);
        }
    }
}
/// expects all urls to start with a './'
/// returns the new url if successfull
fn change_attachments_links(text: &str) -> anyhow::Result<String> {
    let mut modified_lines = Vec::new();

    for line in text.lines() {
        match line.find("![") {
            Some(idx) => {
                match (line[idx..].find('('), line[idx..].find(')')) {
                    (Some(start), Some(end)) => {
                        let start = idx + start + 1; // position of '('
                        let end = idx + end; // position of ')'
                        let url = &line[start + 1..end]; // extract URL
                        let new_url = format!("{}{}", POSTS_DIRECTORY_PATH, url); // create new URL

                        let modified_line =
                            format!("{}{}{}", &line[..start + 1], new_url, &line[end..]); // construct modified line
                        modified_lines.push(modified_line);
                    }
                    _ => return Err(anyhow!("could not find a closing or opening parentheses")),
                }
            }
            None => modified_lines.push(line.to_string()),
        }
    }

    Ok(modified_lines.join("\n"))
}

pub(super) trait TryFromMultipart {
    async fn try_from_multipart(_: Multipart) -> RouterResult<Self>
    where
        Self: Sized;
}

impl TryFromMultipart for UploadBlogPost {
    async fn try_from_multipart(mut mp: Multipart) -> RouterResult<Self> {
        let mut post = UploadBlogPost::default();
        let mut all_files = vec![];

        while let Some(field) = mp.next_field().await? {
            let name = field.name().ok_or(anyhow!("No name on field"))?.to_string();

            match name.as_str() {
                n @ "title" | n @ "subtitle" | n @ "category" => {
                    let bytes: &[u8] = &field.bytes().await?;
                    match n {
                        "title" => post.title = std::str::from_utf8(bytes)?.to_owned(),
                        "subtitle" => post.subtitle = std::str::from_utf8(bytes)?.to_owned(),
                        "category" => {
                            post.category = {
                                let str = std::str::from_utf8(bytes)?.to_owned();
                                if !str.is_empty() {
                                    Some(str)
                                } else {
                                    None
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                "files" => {
                    if let Some(filename) = field.file_name().and_then(|n| Some(n.to_string())) {
                        info!("attachment: {:?}", filename);
                        all_files.push(filename.to_string());
                        post.attachments
                            .push(Attachment::new(&filename, field.bytes().await?.deref()));
                    } else {
                        error!("For some reason a file was passed without a filename");
                    }
                }
                f => return Err(anyhow!("encountered unexpected field: {}", f).into()),
            }
        }
        if all_files
            .iter()
            .find(|n| n.split_once('.').unwrap().1 == "md")
            .is_none()
        {
            error!("No markdown file included in files uploaded");
            return Err(RouterError::from(anyhow!(
                "No markdown file included in files uploaded"
            )));
        }
        Ok(post)
    }
}

impl TryFromMultipart for UpdateBlogPost {
    async fn try_from_multipart(mut mp: Multipart) -> RouterResult<UpdateBlogPost> {
        let mut post = Self::default();
        let mut all_files = vec![];

        while let Some(field) = mp.next_field().await? {
            let name = field.name().ok_or(anyhow!("No name on field"))?.to_string();

            match name.as_str() {
                n @ "title" | n @ "subtitle" | n @ "category" => {
                    let bytes: &[u8] = &field.bytes().await?;
                    match n {
                        "title" => post.title = Some(std::str::from_utf8(bytes)?.to_owned()),
                        "subtitle" => post.subtitle = Some(std::str::from_utf8(bytes)?.to_owned()),
                        "category" => {
                            post.category = {
                                let str = std::str::from_utf8(bytes)?.to_owned();
                                if !str.is_empty() {
                                    Some(str)
                                } else {
                                    None
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                "files" => {
                    if let Some(filename) = field.file_name().and_then(|n| Some(n.to_string())) {
                        info!("attachment: {:?}", filename);
                        all_files.push(filename.to_string());

                        match filename.rsplit_once('.').and_then(|spl| Some(spl.1)) {
                            Some("md") => {
                                post.content = Some(
                                    std::str::from_utf8(field.bytes().await?.deref())?.to_owned(),
                                )
                            }
                            Some(_) => {
                                post.attachments
                                    .push(Attachment::new(&filename, field.bytes().await?.deref()));
                            }
                            None => error!("file without extension passed to form"),
                        }
                    } else {
                        error!("For some reason a file was passed without a filename");
                    }
                }
                f => return Err(anyhow!("encountered unexpected field: {}", f).into()),
            }
        }
        info!("Update built from multipart: {:?}", post);
        Ok(post)
    }
}

mod tests {
    use serde::de::Expected;

    use super::change_attachments_links;

    #[test]
    fn correctly_changes_img_links() {
        let input = r#"this is text
        some more text
        ![img](./img.jpg)
        
        ![img](./img2.jpg)
        [img](./not-an-img)

        this is just me! talking (with parentheses)
        "#;

        let out = change_attachments_links(input).unwrap();
        let expected = r#"this is text
        some more text
        ![img](./public/assets/posts/img.jpg)
        
        ![img](./public/assets/posts/img2.jpg)
        [img](./not-an-img)

        this is just me! talking (with parentheses)
        "#;

        assert_eq!(out, expected);
    }
}
