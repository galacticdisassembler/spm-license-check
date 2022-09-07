use clap::Parser;
use regex::Regex;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the licenses TOML file
    #[clap(short, long)]
    licencefile: String,

    /// Path to the xcworkspace file
    #[clap(short, long)]
    workspace: String,

    /// Enforce lowercase in license names
    #[clap(long, action = clap::ArgAction::SetTrue)]
    lowercase: bool,

    /// swiftpack.co API token (ask @ptrpavlik on twitter for one)
    #[clap(short, long)]
    token: String,
}

#[derive(Deserialize, Debug)]
struct PackageFile {
    pins: Vec<Package>,
    version: i32,
}

#[derive(Deserialize, Debug)]
struct Package {
    identity: String,
    kind: String,
    location: String,
    state: HashMap<Option<String>, Option<String>>,
}

#[derive(Debug, Deserialize)]
struct LicenseFile {
    licenses: HashMap<String, Vec<String>>,
}

fn read_config(file_path: String) -> Result<LicenseFile, Box<dyn Error>> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(toml::from_str(&content)?)
}

fn read_package_resolved(file_path: String) -> Result<PackageFile, Box<dyn Error>> {
    let package_resolved = format!("{}/xcshareddata/swiftpm/Package.resolved", file_path);
    let content = std::fs::read_to_string(package_resolved)?;
    Ok(serde_json::from_str(&content)?)
}

fn form_license_url(source_repo_url: String) -> String {
    let mut url = source_repo_url
        .replace(".git", "")
        .replace("https://github", "https://raw.githubusercontent");
    url.push_str("/master/LICENSE");
    url
}

fn get_version(package_name: &str, token: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://swiftpack.co/api/search?query={}", package_name);
    //    let url = "http://127.0.0.1:8080/";
    let client = reqwest::blocking::Client::new();
    let res = client.get(&url).header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36").header("Authorization", "Basic ".to_owned() +token).send()?;
    let body = res.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let latest_release_name = json[0]["releases"][0]["name"].as_str();
    let latest_release_version = json[0]["latestRelease"].as_str();
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    match latest_release_name {
        Some(_s) => (),
        None => return Ok(latest_release_version.unwrap().to_string()),
    }
    if latest_release_name == latest_release_version {
        return Ok(latest_release_name.unwrap().to_string());
    }
    if re.is_match(latest_release_name.unwrap()) {
        return Ok(latest_release_name.unwrap().to_string());
    }
    Ok(latest_release_version.unwrap().to_string())
}

fn check_license(
    licenses: &HashMap<String, Vec<String>>,
    current_license: &str,
    lowercase: bool,
) -> bool {
    if lowercase {
        licenses["authorized"].contains(&current_license.to_lowercase())
    } else {
        licenses["authorized"].contains(&current_license.to_string())
    }
}

fn get_license_name(license_text: &str) -> &str {
    license_text.split('\n').collect::<Vec<&str>>()[0]
}

fn check_spm_licenses() {
    let args = Args::parse();
    let config = read_config(args.licencefile).unwrap();
    let licenses = config.licenses;
    let packages = read_package_resolved(args.workspace).unwrap();
    for i in packages.pins {
        let full_license_url = form_license_url(i.location);

        let res = match reqwest::blocking::get(full_license_url) {
            Ok(r) => r.text(),
            Err(_err) => {
                return;
            }
        };

        let license_text = res.unwrap();
        let licence_name = get_license_name(&license_text);
        let version = get_version(&i.identity, &args.token).unwrap_or_else(|_| "0.0.0".to_string());
        let check_result = check_license(&licenses, licence_name, args.lowercase);
        let verdict = if check_result { "OK" } else { "FAIL" };
        println!("{} {} {} {}", &i.identity, version, licence_name, verdict);
    }
}

fn main() {
    check_spm_licenses();
}
