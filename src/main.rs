// use std::fs;

fn get_active_window_name() -> String {
    let output = std::process::Command::new("xdotool")
        .arg("getWindowFocus")
        .arg("getWindowName")
        .output()
        .unwrap();

    let x = String::from_utf8(output.stdout).unwrap();
    return x.trim().to_string();
}

fn launch_browser(profile: &String, url: &String) {
    let app = format!("firefox-{profile}");
    let _ = std::process::Command::new("gtk-launch")
        .arg(app)
        .arg(url)
        .spawn();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = args.get(1).unwrap();
    let active_window = get_active_window_name();
    /*
     * Examples of window titles:
     *
     * service-billing - Top Hat - 5 new items - Slack
     * service-billing - Top Hat - Slack
     */
    // fs::write("/tmp/foo", &active_window).expect("Unable to write file");
    // let re = regex::Regex::new(r"^.*-\sTop\sHat\s-\s([0-9]+\snew\sitem[s]?\s-\s)?Slack$").unwrap();

    if active_window.contains("Top Hat") && active_window.contains("Slack") {
        launch_browser(&"tophat".to_string(), url);
    } else {
        launch_browser(&"t".to_string(), url);
    }
}
