/*
 POSIX locale string structure
   "en_xx.UTF_8@blank"
    ^   ^ ^---^ ^------^
    |   |   |      L modifier  - optional
    |   |   L encoding         - optional
    |   L country              - optional
    L language                 - mandatory



 Language File Structure

todo
*/



// Crates
use nu_plugin::{serve_plugin, EvaluatedCall, MsgPackSerializer, Plugin, SimplePluginCommand, PluginCommand, EngineInterface};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Type as NuType, Value as NuValue};
use toml::Value as TomlValue;
use std::env;
use std::fs::{read_to_string};

// modules 
mod ansi_strings;
use ansi_strings::ansify_string;
mod language_toml;
use language_toml::LanguageToml;
mod message_key;
use message_key::MessageKey;
mod posix_language;
use posix_language::PosixLanguage;

// constants
const DEBUG: bool = false;
const LOCALE_LANG: &str = "LANG";

struct Translate;
struct NuTonguesPlugin;
#[allow(dead_code)]
impl Translate {
    fn new() -> Self {
        Self
    }
}

impl SimplePluginCommand for Translate {
    type Plugin = NuTonguesPlugin;
    //the name of the command
    fn name(&self) -> &str {
        "translate"
    }
    //the description of the commands usage
    fn usage(&self) -> &str {
        "Takes in a path to the dir of the translation files via the pipeline and a message key (message.key.format) as a string.
        Returns the desired message translated into the user's language."
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_type(NuType::String, NuType::String)
            .required(
                "msg_key",
                SyntaxShape::String,
                "The name of the message in the translation files.",
            )
            .optional(
                "arguments",
                SyntaxShape::Record(Vec::<(String, SyntaxShape)>::new()),
                "The arguments for the translated string.",
            )
    }

    fn run (
        &self,
        _plugin:    &NuTonguesPlugin,
        _engine:    &EngineInterface,
        call:       &EvaluatedCall,
        input:      &NuValue,
    ) -> Result<NuValue, LabeledError> {
        if DEBUG {
            print!("debug call to translate: ");
            print!("{}", call.nth(0).unwrap().as_str().unwrap());
            print!(" ");
            let args_option = call.nth(1);
            if args_option.is_some() {
                print!("with positional args: ");
                for (arg, val) in args_option.unwrap().as_record().unwrap().iter() {
                    print!{"{}={} ", arg, val.clone().coerce_string().unwrap()};
                }
            }
            println!();
        }
        //gets the environmental variable $LANG and unwraps it
        let mut posix_lang_string: String = env::var(LOCALE_LANG).expect("no $LANG variable");
        let mut path = input
            .as_str()
            .expect("input of translate was not String").to_string();
        //fixes path if its messed up
        if !path.ends_with("/") {
            path += "/";
        }
        //call.nth(0) returns the 1st positional parameter of translate command.
        //as per the .required() function in signature(), it is guaranteed to exist and be a String
        //Then pass it to the MessageKey object constructor
        let msg_key: MessageKey = MessageKey::new(
            call.nth(0)
                .expect("postitional param 0 of translate does not exist") //this should be impossible because of .required()
                .as_str()
                .expect("positional param 0 of translate was not a string").to_string(), //this should also be impossible because SyntaxShape::String of .required()
        );

        //the inverse break condition of the loop. 
        // When fallback is false, it means a translation was successfully found at the message key path
        // When fallback is true, it means we must use the 'fallback' field of the language file to search for the translation in a different file.
        let mut fallback: bool;
        let mut translated_result: String = loop {
            fallback = false;
            let posix_lang: PosixLanguage = PosixLanguage::new(posix_lang_string.clone())
                .expect("$LANG or toml's fallback was not in POSIX format");

            //takes in the dir input and searches all files in it for best translation file matching the user
            //reads it to String
            let best_file_path: String = posix_lang.get_best_file_path(path.clone());

            let language_file_string: String = read_to_string(&best_file_path)
                .expect(format!("failed to open file at path {}", &best_file_path).as_str());
            //generates the translation from that file, reading the whole file
            //optimiztion here possibly
            let language_toml: LanguageToml =
                toml::from_str(language_file_string.as_str()).unwrap();

            //this TomlValue type allows the data to be treated as a table and a string simaltaneously
            let mut toml_value: TomlValue = toml::Value::Table(language_toml.get_messages());
            //loops through the path to get toml_value down to a String
            for key in msg_key.get_path().iter() {
                toml_value = if let Some(thing) = toml_value.get(key) {
                    thing.to_owned()
                } else {
                    //if translation file is incomplete, sets up loop for fallback
                    fallback = true;
                    posix_lang_string = language_toml.get_fallback();
                    break;
                }
            }
            if !fallback {
                break toml_value.to_string();
            }
        };
        if DEBUG {println!("debug got toml file result: {}\nnow processing", translated_result)}
        //variable processing in our string
        let option = call.nth(1);
        if option.is_some() {
            let positionals = option.unwrap();
            for (arg, val) in positionals
                .as_record()
                .expect("positional args index 1 was not a record")
                .iter()
            {
                let parens = &("($".to_string() + &arg + ")").to_owned();
                translated_result = translated_result.replace(parens, val.coerce_string().expect("one of the values in the position arg 1 record was not convertable to string").as_str());
            }
        }
        // For some reason the toml serde crate puts quotes on the ends of every string
        translated_result = translated_result
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();
        //goes through and puts ANSI codes in the string
        let ansi_result = ansify_string(&translated_result);

        Ok(NuValue::string(ansi_result, input.span()))
    
    }
}



impl Plugin for NuTonguesPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(Translate),
        ]
    }
}


fn main() {
    serve_plugin(&NuTonguesPlugin, MsgPackSerializer);
}
