use image::{GenericImageView, ImageError};

fn load_image_info(file_path: &str) -> Result<(), ImageError> {
    // 读取图像
    let img = image::open(file_path)?;

    // 获取图像的宽度和高度
    let (width, height) = img.dimensions();

    // 获取图像的颜色类型
    let color_type = img.color();

    println!("Image loaded successfully!");
    println!("Dimensions: {}x{}", width, height);
    println!("Color type: {:?}", color_type);

    Ok(())
}

fn main() {
    // 替换为你要加载的图像路径
    let file_path = "example.png";

    match load_image_info(file_path) {
        Ok(_) => println!("Image information loaded successfully."),
        Err(e) => eprintln!("Failed to load image: {}", e),
    }
}