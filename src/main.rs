use anyhow::Result;
use clap::Parser;
use fast_qr::qr::QRBuilder;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::QRCode;

/// Simple program to generate image qr
/// from monero address
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// image to save QR code into
    #[arg(short, long)]
    output: Option<String>,

    /// address of monero wallet
    #[arg()]
    address: String
}

fn save_to_image(qrcode: &QRCode, output_file: &str) -> Result<()> {
    ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255, 0]) // Handles transparency
        .fit_width(600)
        .to_file(&qrcode, output_file)?;
    Ok(())
}

fn save_and_print(qrcode: &QRCode) {
    println!("{}", qrcode.to_str());
}

fn main() -> Result<()> {
    let args = Args::parse();
    let address = if args.address.starts_with("monero:") {
        args.address
    } else {
        format!("monero:{}", args.address)
    };

    let qrcode = QRBuilder::new(address)
        .build()?;

    if let Some(o) = args.output {
        save_to_image(&qrcode, &o)?;
    } else {
        save_and_print(&qrcode);
    }

    Ok(())
}
