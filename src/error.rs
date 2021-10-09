#[derive(Debug)]
pub enum ConfigrsError {
    LoadConfig(confy::ConfyError),
    SaveConfig(confy::ConfyError),
    Io(std::io::Error),
    Editor(String),
}
