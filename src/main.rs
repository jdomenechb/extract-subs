use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("mkvextract")
        .args(["-V"])
        .output()
        .expect("mkvextract needs to be installed. Try `sudo apt install mkvtoolnix`.");

    Command::new("ffmpeg")
        .args(["-version"])
        .output()
        .expect("ffmpeg needs to be installed.");

    let args: Vec<String> = env::args().collect();

    let dir;

    if args.len() < 2 {
        dir = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    } else {
        dir = args.get(1).unwrap().to_string();
    }

    if !Path::new(dir.as_str()).exists() {
        panic!("ERROR: Provided path does not exist");
    }

    let paths = fs::read_dir(dir).unwrap();

    for dir_entry in paths {
        let path_buf = dir_entry.unwrap().path();
        let path_str = path_buf.clone().into_os_string().into_string().unwrap();

        if !path_str.to_lowercase().ends_with(".mkv") {
            println!("Ignored non MKV file '{}'", path_str);
            continue;
        }

        let info_result = Command::new("mkvinfo").args([path_str.clone()]).output();

        if let Err(_) = info_result {
            println!("WARNING: Problem treating file '{}'", path_str);
            continue;
        }

        let output = info_result.unwrap();

        if !output.status.success() {
            println!(
                "WARNING: mkvinfo returned exit code '{}' for file '{}'",
                output.status.code().unwrap(),
                path_str
            );

            continue;
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        let output_lines: Vec<&str> = output_str.split("\n").collect();

        let regex_number = Regex::new(r".+\d+.+(\d+)").unwrap();

        let mut number_tmp = None;
        let mut number_final = None;

        for line in output_lines {
            if line.contains("S_TEXT/ASS") {
                number_final = number_tmp.clone();
                break;
            }

            if !line.contains("mkvextract") {
                continue;
            }

            for cap in regex_number.captures_iter(line) {
                number_tmp = Some(cap[1].to_string());
            }
        }

        if number_final.is_none() {
            println!(
                "WARNING: mkvinfo didn't identify any subtitles for file '{}'",
                path_str
            );
            continue;
        }

        let path = path_buf.as_path();
        let path_wo_extension = format!(
            "{}/{}",
            path.parent().unwrap().to_str().unwrap(),
            path.file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
        );

        let sub_file_str = format!("{}.ass", path_wo_extension.clone());

        let extract_result = Command::new("mkvextract")
            .args([
                path_str.clone(),
                "tracks".to_string(),
                format!("{}:{}", number_final.unwrap(), sub_file_str.clone()),
            ])
            .output();

        if let Err(_) = extract_result {
            println!("WARNING: Problem extracting file '{}'", path_str);
            continue;
        }

        let output = extract_result.unwrap();

        if !output.status.success() {
            println!(
                "WARNING: mkvextract returned exit code '{}' for file '{}'",
                output.status.code().unwrap(),
                path_str
            );

            continue;
        }

        let srt_file = format!("{}.srt", path_wo_extension);
        let srt_file_path = Path::new(srt_file.as_str());

        if srt_file_path.exists() {
            fs::remove_file(srt_file_path)
                .expect(format!("ERROR: Cannot remove SRT file {}", srt_file).as_str());
        }

        let conversion_result = Command::new("ffmpeg")
            .args(["-i", sub_file_str.as_str(), srt_file.as_str()])
            .output();

        if let Err(_) = conversion_result {
            println!("WARNING: Problem converting file '{}'", sub_file_str);
            continue;
        }

        let output = conversion_result.unwrap();

        if !output.status.success() {
            println!(
                "WARNING: ffmpeg returned exit code '{}' for file '{}'",
                output.status.code().unwrap(),
                sub_file_str
            );

            continue;
        }

        println!("Processed file '{}'", path_str);
    }
}