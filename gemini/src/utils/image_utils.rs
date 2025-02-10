use anyhow::{bail, Result};
use gemini_api::utils::image::get_image_type_and_base64_string;
use image::codecs::jpeg::JpegEncoder;
use image::GenericImageView;
use reqwest::blocking::Client;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::Path;
use std::sync::LazyLock;

/// 保存图片
pub fn cache_image(url: String, file_name: String) -> Result<()> {
    // 构建文件目录以及名称
    let exe_path = env::current_exe()?;
    let output_path = exe_path.parent().unwrap().join("data").join(file_name);
    create_dir_all(output_path.parent().unwrap())?;
    if url.starts_with("https://") || url.starts_with("http://") {
        // 下载网络图片并压缩
        compress_network_image(url, &output_path, 80)?;
    } else {
        // 压缩图片并保存
        compress_local_image(url, &output_path, 80)?;
    }
    Ok(())
}

/// 压缩图片并保存
pub fn compress_local_image<P>(path: String, file_path: P, quality: u8) -> Result<()>
where
    P: AsRef<Path>,
{
    let img = image::open(path).expect("Failed to open image");
    // 打开输出文件
    let output_file = File::create(file_path).expect("Failed to create output file");
    let writer = BufWriter::new(output_file);
    // 创建 JPEG 编码器
    let mut encoder = JpegEncoder::new_with_quality(writer, quality);
    // 获取图像的宽度、高度和像素数据
    let (width, height) = img.dimensions();
    let pixels = img.as_rgb8().expect("Failed to get RGB8 data");
    // 压缩图像
    encoder
        .encode(pixels, width, height, image::ColorType::Rgb8.into())
        .expect("Failed to encode image");
    Ok(())
}

/// 读取图片
pub fn read_image_cache(file_name: String) -> Result<(String, String)> {
    // 构建文件目录以及名称
    let exe_path = env::current_exe()?;
    let file_path = exe_path.parent().unwrap().join("data").join(file_name);
    get_image_type_and_base64_string(file_path.to_str().unwrap_or_default().into())
}

/// 删除图片
pub fn delete_image_cache(file_name: String) -> Result<()> {
    // 构建文件目录以及名称
    let exe_path = env::current_exe()?;
    let file_path = exe_path.parent().unwrap().join("data").join(file_name);
    if file_path.exists() {
        std::fs::remove_file(file_path)?;
    }
    Ok(())
}
/// 下载网络图片的请求客户端
static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

/// 下载网络图片
pub fn compress_network_image<P>(path: String, file_path: P, quality: u8) -> Result<()>
where
    P: AsRef<Path>,
{
    let response = CLIENT.get(path).send()?;
    if response.status().is_success() {
        let bytes = response.bytes()?;
        let img = image::load_from_memory(&bytes).expect("Failed to load image from memory");
        // 打开输出文件
        let output_file = File::create(file_path).expect("Failed to create output file");
        let writer = BufWriter::new(output_file);
        // 创建 JPEG 编码器
        let mut encoder = JpegEncoder::new_with_quality(writer, quality);
        // 获取图像的宽度、高度和像素数据
        let (width, height) = img.dimensions();
        let pixels = img.as_rgb8().expect("Failed to get RGB8 data");
        // 压缩图像
        encoder
            .encode(pixels, width, height, image::ColorType::Rgb8.into())
            .expect("Failed to encode image");
        Ok(())
    } else {
        bail!("Failed to download image")
    }
}
