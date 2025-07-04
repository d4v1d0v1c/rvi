use std::path::Path;
use rmore::input::Input;

pub fn new_stdin_input(name: Option<&Path>) -> Input {
    named(Input::stdin(), name)
}

pub fn new_file_input<'a>(file: &'a Path, name: Option<&'a Path>) -> Input<'a> {
    named(Input::ordinary_file(file), name.or(Some(file)))
}

fn named<'a>(input: Input<'a>, name: Option<&Path>) -> Input<'a> {
    if let Some(provided_name) = name {
        let mut input = input.with_name(Some(provided_name));
        input.description_mut().set_kind(Some("File".to_owned()));
        input
    } else {
        input
    }
}
