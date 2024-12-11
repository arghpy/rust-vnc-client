use std::fs;
use iced::widget::PickList;
use iced::Element;

const HOSTS_FILE: &str = "test_input_combobox.txt";

fn main() -> iced::Result {
    iced::run("VNC Client", Client::update, Client::view)
}

fn read_file(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| String::from(line))
        .collect()
}

struct Client {
    hosts: Vec<String>,
    selected_host: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    SelectedHost(String),
}

impl Client {
    fn new() -> Self {
        Client { 
            hosts: read_file(HOSTS_FILE),
            selected_host: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectedHost(value) => {
                self.selected_host = Some(value);
            }
        }
    }
    fn view(&self) -> Element<Message> {
        PickList::new(
            &self.hosts[..],
            self.selected_host.clone(),
            Message::SelectedHost
        ).placeholder("Select/Enter an option").into()
    }

}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

