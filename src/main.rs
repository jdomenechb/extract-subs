mod terminal;

use crate::terminal::{format_error, print_error, print_success, print_warning};
use regex::Regex;
use std::env;
use std::fs;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

fn check_requirements() {
    Command::new("mkvextract").args(["-V"]).output().expect(
        format_error("mkvextract needs to be installed. Try `sudo apt install mkvtoolnix`.")
            .as_str(),
    );

    Command::new("ffmpeg")
        .args(["-version"])
        .output()
        .expect("ffmpeg needs to be installed.");
}

fn determine_dir_to_extract() -> String {
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
        panic!(
            "{}",
            format_error("Provided path does not exist.",).as_str()
        );
    }

    dir
}

fn main() {
    check_requirements();

    let dir = determine_dir_to_extract();
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
            print_error(format!("Problem treating file '{}'", path_str));
            continue;
        }

        let output = info_result.unwrap();

        if !output.status.success() {
            print_error(format!(
                "mkvinfo returned exit code '{}' for file '{}'",
                output.status.code().unwrap(),
                path_str
            ));

            continue;
        }

        let output_str = String::from_utf8(output.stdout).unwrap();
        let output_lines: Vec<&str> = output_str.split("\n").collect();

        let regex_number = Regex::new(r".+\d+.+(\d+)").unwrap();

        let mut number_tmp = None;
        let mut number_final = None;
        let mut is_ass = false;

        for line in output_lines {
            if line.contains("S_TEXT/ASS") {
                number_final = number_tmp.clone();
                is_ass = true;
                break;
            }

            if line.contains("S_TEXT/UTF8") {
                number_final = number_tmp.clone();
                is_ass = false;
            }

            if !line.contains("mkvextract") {
                continue;
            }

            for cap in regex_number.captures_iter(line) {
                number_tmp = Some(cap[1].to_string());
            }
        }

        if number_final.is_none() {
            print_warning(format!(
                "mkvinfo didn't identify any subtitles for file '{}'",
                path_str
            ));
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

        let srt_file = format!("{}.srt", path_wo_extension);
        let srt_file_path = Path::new(srt_file.as_str());

        if srt_file_path.exists() {
            remove_file(srt_file_path)
                .expect(format!("ERROR: Cannot remove SRT file {}", srt_file).as_str());
        }

        let sub_extension = if is_ass { "ass" } else { "srt" };
        let sub_file_str = format!("{}.{}", path_wo_extension.clone(), sub_extension);

        let extract_result = Command::new("mkvextract")
            .args([
                path_str.clone(),
                "tracks".to_string(),
                format!("{}:{}", number_final.unwrap(), sub_file_str.clone()),
            ])
            .output();

        if let Err(_) = extract_result {
            print_error(format!("Problem extracting file '{}'", path_str));
            continue;
        }

        let output = extract_result.unwrap();

        if !output.status.success() {
            print_error(format!(
                "mkvextract returned exit code '{}' for file '{}'",
                output.status.code().unwrap(),
                path_str
            ));

            continue;
        }

        if is_ass {
            let conversion_result = Command::new("ffmpeg")
                .args(["-i", sub_file_str.as_str(), srt_file.as_str()])
                .output();

            if let Err(_) = conversion_result {
                print_error(format!("Problem converting file '{}'", sub_file_str));
                continue;
            }

            let output = conversion_result.unwrap();

            if !output.status.success() {
                print_error(format!(
                    "ffmpeg returned exit code '{}' for file '{}'",
                    output.status.code().unwrap(),
                    sub_file_str
                ));

                continue;
            }

            if let Err(_) = remove_file(sub_file_str.as_str()) {
                print_error(format!("could not delete file '{}'", sub_file_str));
            }
        }

        print_success(format!("Processed file '{}'", path_str));
    }
}
