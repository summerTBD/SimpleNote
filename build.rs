use std::io::Write as _;

fn main() {
    // 仅 Windows 平台嵌入图标
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo 应设置 CARGO_CFG_TARGET_OS");
    if target_os != "windows" {
        return;
    }

    // ---- 第一步：把 PNG 转成 ICO ----
    let png_bytes = include_bytes!("assets/icon-512.png");
    let img = image::load_from_memory(png_bytes).expect("无法加载图标 PNG");

    let out_dir = std::env::var("OUT_DIR").expect("Cargo 应设置 OUT_DIR");
    let ico_path = format!("{out_dir}/icon.ico");

    // 生成含多个分辨率的 ICO（16/32/48/256 是 Windows 标准尺寸）
    let ico_bytes = img_to_ico(&img);
    std::fs::write(&ico_path, &ico_bytes).expect("无法写入 ICO");

    // ---- 第二步：将 ICO 嵌入 exe 资源 ----
    let mut res = winres::WindowsResource::new();
    res.set_icon(&ico_path);
    if let Err(e) = res.compile() {
        // 跨平台编译场景（如 Linux 交叉编译 Windows）静默跳过
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{out_dir}/build_warning.txt"))
            .expect("无法创建构建警告日志");
        writeln!(f, "winres 编译失败（可忽略）: {e}").ok();
        println!("cargo:warning=无法嵌入 exe 图标（可能缺少 windres）: {e}");
    }
}

/// 将图像转为含多尺寸的 ICO 字节
fn img_to_ico(img: &image::DynamicImage) -> Vec<u8> {
    use image::codecs::ico::IcoFrame;

    let sizes = [16u32, 32, 48, 256];

    let frames: Vec<IcoFrame<'_>> = sizes
        .iter()
        .map(|&s| {
            let resized = img.resize_exact(s, s, image::imageops::FilterType::Lanczos3);
            let rgba = resized.to_rgba8();
            let (w, h) = rgba.dimensions();
            IcoFrame::as_png(rgba.as_raw(), w, h, image::ExtendedColorType::Rgba8)
                .expect("ICO 帧编码失败")
        })
        .collect();

    let mut buf = std::io::Cursor::new(Vec::new());
    let encoder = image::codecs::ico::IcoEncoder::new(&mut buf);
    encoder.encode_images(&frames).expect("ICO 编码失败");
    buf.into_inner()
}
