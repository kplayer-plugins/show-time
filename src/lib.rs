extern crate kplayer_rust_wrap;

use kplayer_rust_wrap::kplayer;

struct ShowTime {}
impl ShowTime {
    fn new() -> Self {
        ShowTime {}
    }
}

impl kplayer::plugin::BasePlugin for ShowTime {
    fn get_name(&self) -> String {
        String::from("show-text")
    }
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("text=%{localtime}"));
        args.push(String::from("fontsize=17"));
        args.push(String::from("fontcolor=white"));
        args.push(String::from("fontfile=resource/font.ttf"));
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));

        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("drawtext")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(&self, _args: &Vec<String>) -> std::result::Result<bool, &'static str> {
        for str in _args {
            let sp: Vec<&str> = str.split('=').collect();
            if sp.len() < 2 {
                self.print_log(
                    kplayer::util::os::PrintLogLevel::ERROR,
                    format!("validate args failed arg string: {}", str).as_str(),
                );
                return Err("args format error");
            }

            // validate font file exist
            if sp[0] == "fontfile" {
                if !kplayer::util::os::file_exist(sp[1].to_string()) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("font file not eixst: {}", str).as_str(),
                    );
                    return Err("font file not exist");
                }
                continue;
            }

            // validate text invalid
            if sp[0] == "text" {
                if sp[1] != "%{localtime}" {
                    return Err("text argument can not be custom");
                }
                continue;
            }
        }

        Ok(true)
    }
}

kplayer_rust_wrap::export!(ShowTime);
