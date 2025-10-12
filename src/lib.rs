use std::{collections::HashMap, time::{SystemTime, self, Duration}, fs};

pub struct Theme {
    background_rectangle_primary: String,
    background_rectangle_stroke: String,
    language: String,
    primary_font: String,
    primary_text_color: String,
    primary_text: String,
    session_font: String,
    session_text_color: String,
    editor_font: String,
    editor_color: String,
    editor_text: String,
}

impl Theme {
    pub fn dark() -> Theme {
        Theme {
            background_rectangle_primary: "rgba(22,27,34)".to_string(),
            background_rectangle_stroke: "#484f58".to_string(),
            language: "none".to_string(),
            primary_font: "Ubuntu".to_string(),
            primary_text_color: "white".to_string(),
            primary_text: "Idling".to_string(),
            session_font: "Ubuntu".to_string(),
            session_text_color: "#ccc".to_string(),
            editor_font: "Ubuntu".to_string(),
            editor_color: "grey".to_string(),
            editor_text: "Visual Studio Code".to_string(),
        }
    }

    pub fn light() -> Theme {
        Theme {
            background_rectangle_primary: "white".to_string(),
            background_rectangle_stroke: "#008fff".to_string(),
            language: "none".to_string(),
            primary_font: "Ubuntu".to_string(),
            primary_text_color: "#008fff".to_string(),
            primary_text: "Idling".to_string(),
            session_font: "Ubuntu".to_string(),
            session_text_color: "#007fee".to_string(),
            editor_font: "Ubuntu".to_string(),
            editor_color: "#557faa".to_string(),
            editor_text: "Visual Studio Code".to_string(),
        }
    }
}

pub fn parse_args(args: Vec<String>) -> Result<HashMap<String, String>, String> {
    args.iter()
        .skip(1)
        .try_fold(HashMap::new(), |mut map, arg| {
            if let Some((key, value)) = arg.split_once('=') {
                map.insert(key.to_string(), value.to_string());
                Ok(map)
            } else {
                Err(format!("Invalid argument: {}", arg)) // handle invalid formats
            }
        })
}

pub fn make_theme(args: HashMap<String, String>) -> Theme {
    let mut theme = match args.get("theme") {
        Some(x) if x == "light" => Theme::light(),
        Some(_) | None => Theme::dark(),
    };

    let arg_keys = [
        "bg-primary",
        "bg-stroke",
        "lang",
        "primary-color",
        "primary-font",
        "primary-text",
        "session-color",
        "session-font",
        "session-text-time",
        "editor-color",
        "editor-font",
        "editor-text",
    ];

    for key in arg_keys {
        if let Some(x) = args.get(key) {
            match key {
                "bg-primary" => theme.background_rectangle_primary = x.clone(),
                "bg-stroke" => theme.background_rectangle_stroke = x.clone(),
                "lang" => theme.language = x.clone(),
                "primary-color" => theme.primary_text_color = x.clone(),
                "primary-font" => theme.primary_font = x.clone(),
                "primary-text" => theme.primary_text = x.clone(),
                "session-color" => theme.session_text_color = x.clone(),
                "session-font" => theme.session_font = x.clone(),
                "editor-color" => theme.editor_color = x.clone(),
                "editor-font" => theme.editor_font = x.clone(),
                "editor-text" => theme.editor_text = x.clone(),
                &_ => {}
            }
        }
    }

    theme
}

pub fn parse_time(start_time: u64) -> String {
    let time_now = match SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not get unix time epochs");
            eprintln!("{:?}", e);
            std::process::exit(0);
        },
    };

    let duration = time_now - Duration::from_secs(start_time);

    // if more than an hour, return as hours. else return as minutes

    if duration.as_secs() < 60{
        return "Less than 1 min".to_string();
    }
    else if duration.as_secs() < 3600{
        return (duration.as_secs() / 60).to_string() + " mins";
    }
    else{
        return (duration.as_secs() / 3600).to_string() + " hours";
    }
}

// could be in it's own module
fn find_language_svg(language: &str) -> String{
    let language_svgs = HashMap::from([
        ("js", "svg/js.svg"),
        ("javascript", "svg/js.svg"),
        ("ts", "svg/ts.svg"),
        ("typescript", "svg/ts.svg"),
        ("javascriptreact", "svg/react.svg"),
        ("typescriptreact", "svg/react.svg"),
        ("react", "svg/react.svg"),
        ("rust", "svg/rust.svg"),
        ("go", "svg/go.svg"),
        ("c", "svg/c.svg"),
        ("cpp", "svg/cpp.svg"),
        ("c++", "svg/cpp.svg"),
        ("csharp", "svg/csharp.svg"),
        ("c#", "svg/csharp.svg"),
        ("none", "svg/vscode.svg")
    ]);

    let file_path = language_svgs.get(language).unwrap_or(&"svg/vscode.svg");

    match fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Fetching language svg failed: {}", e);
            std::process::exit(1);
        },
    }

}

pub fn make_svg(theme: Theme, session_time: &str) -> String {
    let language_svg = find_language_svg(&theme.language.to_lowercase());
    let svg = format!(
        "
   <svg width=\"450\" height=\"130\"
   viewBox=\"0 0 450 130\"
        fill=\"none\"
        xmlns=\"http://www.w3.org/2000/svg\"
        xmlns:xlink=\"http://www.w3.org/1999/xlink\"
        role=\"img\">
    <style>
    *{{
        font-family: 'Segoe UI', Ubuntu, monospace
    }}
    </style>
    <defs>
  <clipPath id=\"horizontalClip\" x=\"20\" y=\"20\" width=\"500\" height=\"200\" >
    <rect x=\"90\" y=\"20\" width=\"300\" height=\"50\"></rect>
    </clipPath>
</defs>

    <rect width=\"450\" height=\"130\" fill=\"{}\" 
    stroke=\"{}\" stroke-width=\"1\" fill-opacity=\"1\"
    stroke-opacity=\"1\" x=\"0\" y=\"0\" rx=\"10px\" ry=\"10px\">
    </rect>

    {}
    <text x=\"110\" y=\"45\" font-size=\"20\" 
    clip-path=\"url(#horizontalClip)\"
    font-weight=\"bold\" font-family=\"{}\" fill=\"{}\">
    {}
    </text>
    <text x=\"110\" y=\"70\" font-size=\"17px\" font-family=\"{}\" fill=\"{}\">
    Last session: {}
    </text>
    <text x=\"110\" y=\"90\" font-size=\"12px\" font-family=\"{}\" fill=\"{}\">
    {}
    </text>
   </svg> 
    ",
        theme.background_rectangle_primary,
        theme.background_rectangle_stroke,
        language_svg,
        theme.primary_font,
        theme.primary_text_color,
        theme.primary_text,
        theme.session_font,
        theme.session_text_color,
        session_time,
        theme.editor_font,
        theme.editor_color,
        theme.editor_text
    );

    println!("{}", svg);
    svg
}
