use tesseract::Tesseract;

fn main() -> () {
    print!("-----IMAGE ONE-----");
    image_to_text("./assets/image.jpg");
    print!("-----IMAGE TWO-----");
    image_to_text("./assets/image2.jpg");
    print!("-----IMAGE THREE-----");
    image_to_text("./assets/image3.jpeg");
    print!("-----IMAGE FOUR-----");
    image_to_text("./assets/image4.jpeg");
    print!("-----IMAGE FIVE-----");
    image_to_text("./assets/image5.jpeg");
}

fn image_to_text(path: &str) {
    let tesseract = Tesseract::new(None, Some("eng")).unwrap();

    let text = tesseract
        .set_image(path).expect("REASON")
        .recognize().unwrap()
        .get_text().unwrap();
    println!("{}", text);
}