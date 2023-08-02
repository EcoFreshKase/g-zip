//#![windows_subsystem = "windows"]

use std::env;
use std::path::Path;
use druid::{WindowDesc, AppLauncher, Selector, theme};

mod convert_utils;
mod gui_builder;

use gui_builder::decode_encode;

const WINDOW_SIZE: (f64, f64) = (599.0 *1.3, 337.0 *1.3); //Size of the window (width, height)
const ERROR: Selector = Selector::new("ERROR WHILE CONVERTING");
const NEW_LOADING_WINDOW: Selector = Selector::new("CREATE A NEW LOADING WINDOW");
const START_CONVERSION: Selector = Selector::new("START CONVERSION");
const GLOBAL_UPDATE: Selector = Selector::new("UPDATE GLOBAL");

fn main() {

    //let args: Vec<String> = env::args().collect();
    let args: Vec<String> = vec![String::from("C:\\Users\\alexa\\Desktop\\Coding Projekte\\g-zip\\dummy_file.txt")];
    //let args: Vec<String> = vec![String::from("C:\\Users\\alexa\\OneDrive\\Bilder\\Anime\\I've been killing slimes for 300 years\\91139172_p0_master1200.jpg")]; //big image
    //let args: Vec<String> = vec![String::from("D:\\Bilder\\Insta\\convert.7z")]; //very large file
    
    //let args: Vec<String> = vec![String::from("C:\\Users\\alexa\\Desktop\\Coding Projekte\\g-zip\\dummy_file.fasta")]; // FASTA file
    let path = args.get(0).unwrap(); //for test
    //let path = args.get(1).unwrap(); //for release

    //default values for the start of the application
    let default_save_extension= ".fasta";
    let default_save_path = {
        let extension_index = {
            let mut output = None;
            for (index, char) in path.chars().rev().enumerate() { //iterate through the path from behind
                if char == '.' {
                    output = Some(path.len() - index - 1);
                    break;
                } else if char == '\\' {
                    output = Some(path.len());
                    break;
                }
            }
            output.expect(&format!("Path is not valid: {}", path))
        };
        path[..extension_index].to_owned() + default_save_extension
    };
    let default_algorithm_type = {
        let path = Path::new(path);
        let mut output = decode_encode::AlgorithmType::Encode;

        if let Some(extension) = path.extension() {
            if extension == "fasta" {
                output = decode_encode::AlgorithmType::Decode;
            }
        }
        output
    };

    //GUI creation
    let main_window = WindowDesc::new(gui_builder::build_ui())
        .title("g-zip")
        .resizable(false)
        .window_size(WINDOW_SIZE);
    let initial_state = gui_builder::AppState::AppState {
        file_path: path.to_owned(),
        save_path: default_save_path,
        error_correcting: gui_builder::error_correcting::ErrorCorrecting::None,
        algorithm_type: default_algorithm_type,
        decode_algorithm: None,
        encode_algorithm: None,
        calculating: 0.0,
        calculating_msg: String::new(),
        error_msg: String::new(),

        header_file_name: false,
        header_file_ext: false,
        header_file_size: false,
        header_used_algorithm: true,
        header_used_error_correcting: true,

        debugging: false,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(|env, _| {
            env.set(theme::WIDE_WIDGET_WIDTH, 200.0);
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}