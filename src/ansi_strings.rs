use nu_ansi_term::Color::{Fixed, Rgb};
use nu_ansi_term::*;

fn base_10_str_to_u8(string: &str) -> u8 {
    u8::from_str_radix(string, 10)
        .expect("string that was expected to be a u8 was formatted incorrectly")
}

pub fn ansify_string<'a>(input_string: &'a String) -> String {
    let input_string_split: Vec<&str> = input_string.split("(ansi ").collect();
    let mut ansi_result =
        Vec::<AnsiGenericString<'static, str>>::with_capacity(input_string_split.len());
    for i in 0..input_string_split.len() {
        ansi_result.push(if i == 0 {
            Color::Default.paint(input_string_split[i])
        } else {
            let seperate_command_from_rest: Vec<&str> =
                input_string_split[i].splitn(2, ")").collect();
            let command = seperate_command_from_rest[0];
            let the_rest = seperate_command_from_rest[1];

            let mut style = Style::new();

            if command.contains("bold") {
                style = style.bold()
            }
            if command.contains("dimmed") {
                style = style.dimmed()
            }
            if command.contains("italic") {
                style = style.italic()
            }
            if command.contains("underline") {
                style = style.underline()
            }
            if command.contains("strikethrough") {
                style = style.strikethrough()
            }
            if command.contains("hidden") {
                style = style.hidden()
            }
            if command.contains("blink") {
                style = style.blink()
            }

            style = ansi_compute_and_add_color(command, &style);

            if command.contains("reverse") {
                style = style.reverse()
            }

            style.paint(the_rest)
        });
    }

    let ansi_strings_copy: AnsiStrings<'a> = AnsiGenericStrings(ansi_result.leak());
    format!("{}", ansi_strings_copy)
}

fn ansi_compute_and_add_color(command: &str, style: &Style) -> Style {
    let mut result = style.clone();
    if command.contains("color") {
        let mut counter = 0;
        for (i, _match_word) in command.match_indices("color") {
            let mut color_args = command
                .get(
                    (i + 6)
                        ..(command.match_indices(']').collect::<Vec<(usize, &str)>>()[counter].0),
                )
                .unwrap()
                .trim_end_matches(']')
                .trim_start_matches('[');
            let is_foreground: bool = !color_args.starts_with("bg");

            color_args = color_args.trim_start_matches("bg;");
            color_args = color_args.trim_start_matches("fg;");

            let color_args_split: Vec<&str> = color_args.split(";").collect();

            let color = if color_args_split.len() < 3 {
                match ansi_color_from_str(color_args_split[0]) {
                    Some(color) => color,
                    None => Fixed(base_10_str_to_u8(color_args_split[0])),
                }
            } else {
                Rgb(
                    base_10_str_to_u8(color_args_split[0]),
                    base_10_str_to_u8(color_args_split[1]),
                    base_10_str_to_u8(color_args_split[2]),
                )
            };

            if is_foreground {
                result = style.fg(color);
            } else {
                result = style.on(color);
            }
            counter += 1;
        }
    }
    result
}




pub fn ansi_color_from_str(string: &str) -> Option<Color> {
    match string.trim().to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "blue" => Some(Color::Blue),
        "cyan" => Some(Color::Cyan),
        "darkgray" => Some(Color::DarkGray),
        "default" => Some(Color::Default),
        "green" => Some(Color::Green),
        "lightblue" => Some(Color::LightBlue),
        "lightcyan" => Some(Color::LightCyan),
        "lightgray" => Some(Color::LightGray),
        "lightgreen" => Some(Color::LightGreen),
        "lightmagenta" => Some(Color::LightMagenta),
        "lightpurple" => Some(Color::LightPurple),
        "lightred" => Some(Color::LightRed),
        "lightyellow" => Some(Color::LightYellow),
        "magenta" => Some(Color::Magenta),
        "purple" => Some(Color::Purple),
        "red" => Some(Color::Red),
        "white" => Some(Color::White),
        "yellow" => Some(Color::Yellow),
        _ => None,
    }
}