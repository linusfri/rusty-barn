use std::{collections::HashMap, thread};
use image::{io::Reader as ImageReader, DynamicImage};
use axum::{
    extract::Multipart,
    response::Html
    
};

pub async fn deepfry(mut image_data: Multipart) -> Html<String> {
    let mut files: HashMap<String, DynamicImage> = HashMap::new();

    while let Some(field) = image_data.next_field().await.unwrap_or(None) {
        let file_name = String::from(field.file_name().unwrap());
        let byte_img = field.bytes().await;
        let bytes = match byte_img {
            Ok(image) => image,
            Err(_) => continue
                
        };
        let img = ImageReader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    
        files.insert(String::from(file_name), img);
    };
    
    let image_html: String = files.iter().map(|(name, image)| {
        format!("<li>{}</li>", name)
    }).collect();

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

    Html(image_html)
}