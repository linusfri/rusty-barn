use std::{collections::HashMap, thread, vec};
use image::{io::Reader as ImageReader, DynamicImage};
use axum::{
    extract::{multipart, Multipart},
    response::{Html, Redirect}
};
use regex::Regex;
use tera::Context;
use crate::http::TEMPLATES;

use super::error::AppResult;

pub struct ImageAndFilename {
    pub img: DynamicImage,
    pub file_name: String
}

pub async fn deepfry(mut image_data: Multipart) -> AppResult<Html<String>> {
    let mut files: HashMap<String, DynamicImage> = HashMap::new();

    while let Some(field) = image_data.next_field().await.unwrap_or(None) {
        let img_data = get_dynamic_img_and_filename(field).await?;
    
        files.insert(String::from(img_data.file_name), img_data.img);
    };

    process(files).await;

    let mut context = Context::new();
    let files = get_uploaded_image_uris().await?;
    context.insert(String::from("images"), &files);

    Ok(TEMPLATES.render("gallery.html", &context)
        .map(|s| Html(s))?
    )
}

pub async fn upload(mut image_data: Multipart) -> AppResult<Redirect> {
    let mut image_thread_handles: vec::Vec<thread::JoinHandle<()>> = vec![];

    while let Some(field) = image_data.next_field().await.unwrap_or(None) {
        let img_data = get_dynamic_img_and_filename(field).await?;

        image_thread_handles.push(thread::spawn(move || {
            img_data.img.save(format!("src/public/assets/{}", img_data.file_name)).unwrap();
        }));
    };

    image_thread_handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    let mut context = Context::new();
    let files = get_uploaded_image_uris().await?;
    context.insert(String::from("images"), &files);

    Ok(Redirect::to("/"))
}

pub async fn get_uploaded_image_uris() -> AppResult<Vec<String>> {
    let file_names = std::fs::read_dir("src/public/assets")?
        .filter(|file| {
            let path_buf = file.as_ref().unwrap().path();
            let path = path_buf.as_path();

            file.is_ok() && std::path::Path::is_file(path) && is_file_allowed_image(path)
        }) 
        .map(|file| {
            format!("assets/{}", file.unwrap().path().file_name().unwrap().to_str().unwrap())
        }).collect::<Vec<String>>();
    
    Ok(file_names)
}

fn is_file_allowed_image(path: &std::path::Path) -> bool {
    let regex = Regex::new(r"(png|jpg|jpeg|webp)").unwrap();

    match path.extension() {
        Some(ext) => regex.is_match(ext.to_str().unwrap_or("")),
        None => false
    }
}

async fn process(files: HashMap<String, DynamicImage>) {
    let mut join_handles: Vec<thread::JoinHandle::<()>> = vec![];

    files.into_iter().for_each(|(name, file)| {
        let handle = thread::spawn(move || {
            let modified = file.adjust_contrast(100.0).huerotate(90).unsharpen(500.0, -500).filter3x3(&[
                -50.0, -1.0, 10.0,
                80.0, 5.0, -120.0,
                20.0, -4.0, -40.0
            ]);
    
            modified.save(format!("src/public/assets/{}.png", name)).unwrap();
        });

        join_handles.push(handle);
    });

    join_handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });
}

async fn get_dynamic_img_and_filename<'a>(field: multipart::Field<'a>) -> AppResult<ImageAndFilename> {
    let file_name = String::from(field.file_name().unwrap());

    let bytes = field.bytes().await?;

    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    Ok(ImageAndFilename {
        img,
        file_name
    })
}