use std::env;
use std::path::PathBuf;
use std::process;

use image_to_pure_css::convert_image_to_css;

fn format_with_thousands(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

fn parse_args(argv: &[String]) -> (PathBuf, Option<u32>, u8, PathBuf) {
    let args: Vec<&String> = argv.iter().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "Usage: image-to-pure-css <image> [--width N] [--tolerance N] [--output file.txt]"
        );
        process::exit(1);
    }

    let input_path = PathBuf::from(args[0]);
    let mut width: Option<u32> = None;
    let mut tolerance = 0;
    let mut output: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                i += 1;
                if i < args.len() {
                    width = args[i].parse::<u32>().ok();
                }
            }
            "--tolerance" => {
                i += 1;
                if i < args.len() {
                    tolerance = args[i].parse().unwrap_or(0);
                }
            }
            "--output" => {
                i += 1;
                if i < args.len() {
                    output = Some(PathBuf::from(args[i]));
                }
            }
            _ => {}
        }
        i += 1;
    }

    let output = output.unwrap_or_else(|| {
        let stem = input_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let new_stem = stem.rsplit_once('.').map(|(s, _)| s).unwrap_or(&stem);
        PathBuf::from(format!("{new_stem}.txt"))
    });

    (input_path, width, tolerance, output)
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let (input_path, width, tolerance, output) = parse_args(&argv);

    println!("Reading: {}", input_path.display());
    let mut file = match std::fs::File::create(&output) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open output: {e}");
            process::exit(1);
        }
    };
    let mut writer = std::io::BufWriter::new(&mut file);
    if let Err(e) = convert_image_to_css(
        &input_path.to_string_lossy(),
        image_to_pure_css::ConvertImageOptions { width, tolerance },
        &mut writer,
    ) {
        eprintln!("{e}");
        process::exit(1);
    };
    drop(writer);
    let size = file.metadata().unwrap().len();

    let size_kb = size as f64 / 1024.0;
    println!(
        "Output: {:.1} KB ({} chars)",
        size_kb,
        format_with_thousands(size)
    );

    println!("Written: {}", output.display());
}
