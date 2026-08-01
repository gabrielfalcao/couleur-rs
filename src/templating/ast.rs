use chrono::Utc;
use pest::iterators::Pair;

use crate::color as colors;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Color(Color),
    Contrast(crate::contrast::Contrast),
    Layer(crate::layer::Layer),
    Reset(crate::reset::Reset),
    Unhandled(String),
    InvalidMarkup(InvalidMarkupToken),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaletteColor {
    pub palette_name: String,
    pub color_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    FromPalette(PaletteColor), // (palette_name: String, color_name: String)
    Named(String),
    Terminal(Layer),
    Rgb(crate::color::Color),
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Color {
    pub fn to_str(&self) -> String {
        match self {
            Color::FromPalette(value) => {
                value.to_string() // PaletteColor
            }
            Color::Named(value) => {
                value.to_string() // String
            }
            Color::Terminal(value) => {
                value.to_string() // Layer
            }
            Color::Rgb(value) => {
                value.to_string() // crate::color::Color
            }
        }
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Color> {
        let pair = pair.clone();
        let rule = pair.clone().as_rule();
        let mut pairs = pair.clone().into_inner();

        pairs.next().unwrap(); // consume "escape_character"
        let variable = match &rule {
            Rule::variable_strftime => {
                Color::Strftime(pairs.next().expect("strftime").as_span().as_str().to_string()) // `\D{format}` // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
            }
            Rule::variable_ascii_octal_code => {
                let value = pairs.next().expect("octal").as_span().as_str().to_string();
                let code = u8::from_str_radix(value.as_str(), 8).expect("octal");
                Color::AsciiOctalCode(code) // `\nnn` // The character whose ASCII code is the octal value nnn.
            }
            Rule::variable_bell => {
                Color::Bell // `\a` // A bell character.
            }
            Rule::variable_date_weekday => {
                Color::DateWeekday // `\d` // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
            }

            Rule::variable_escape_character => {
                Color::EscapeCharacter // `\e` // An escape character.
            }
            Rule::variable_hostname_short => {
                Color::HostnameShort // `\h` // The hostname, up to the first ‘.’.
            }
            Rule::variable_hostname => {
                Color::Hostname // `\H` // The hostname.
            }
            Rule::variable_jobs_count => {
                Color::JobsCount // `\j` // The number of jobs currently managed by the shell.
            }
            Rule::variable_shell_device_name => {
                Color::ShellDeviceName // `\l` // The basename of the shell’s terminal device name.
            }
            Rule::variable_newline => {
                Color::Newline // `\n` // A newline.
            }
            Rule::variable_carriage_return => {
                Color::CarriageReturn // `\r` // A carriage return.
            }
            Rule::variable_shell_name => {
                Color::ShellName // `\s` // The name of the shell, the basename of $0 (the portion following the final slash).
            }
            Rule::variable_time_24h_format => {
                Color::Time24hFormat // `\t` // The time, in 24-hour HH:MM:SS format.
            }
            Rule::variable_time_12h_format => {
                Color::Time12hFormat // `\T` // The time, in 12-hour HH:MM:SS format.
            }
            Rule::variable_time_12h_ampm => {
                Color::Time12hAmpm // `\@` // The time, in 12-hour am/pm format.
            }
            Rule::variable_time_24h_short => {
                Color::Time24hShort // `\A` // The time, in 24-hour HH:MM format.
            }
            Rule::variable_username => {
                Color::Username // `\u` // The username of the current user.
            }
            Rule::variable_bash_version => {
                Color::BashVersion // `\v` // The version of Bash (e.g., 2.00)
            }
            Rule::variable_bash_version_full => {
                Color::BashVersionFull // `\V` // The release of Bash, version + patchlevel (e.g., 2.00.0)
            }
            Rule::variable_pwd_short => {
                Color::PwdShort // `\w` // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
            }
            Rule::variable_pwd_long => {
                Color::PwdLong // `\W` // The basename of $PWD, with $HOME abbreviated with a tilde.
            }
            Rule::variable_history_number => {
                Color::HistoryNumber // `\!` // The history number of this command.
            }
            Rule::variable_command_number => {
                Color::CommandNumber // `\#` // The command number of this command.
            }
            Rule::variable_prompt_end => {
                Color::PromptEnd // `\$` // If the effective uid is 0, #, otherwise $.
            }
            Rule::variable_backslash => {
                Color::Backslash // `\\` // A backslash.
            }
            Rule::variable_begin_nonprinting => {
                Color::BeginNonprinting // `\[` // Begin a sequence of non-printing characters.
            }
            Rule::variable_end_nonprinting => {
                Color::EndNonprinting // `\]` // End a sequence of non-printing characters.
            }
            _ => {
                unreachable!("{:#?}", pair);
            }
        };
        Ok(variable)
    }

    pub fn repr(&self) -> String {
        format!(
            "\\{}",
            match self {
                Color::AsciiOctalCode(c) => format!("{:03o}", c), // The character whose ASCII code is the octal value nnn.
                Color::Strftime(f) => format!("D{{{}}}", f), // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
                Color::Bell => format!("a"),                 // A bell character.
                Color::DateWeekday => format!("d"),          // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
                Color::EscapeCharacter => format!("e"),      // An escape character.
                Color::HostnameShort => format!("h"),        // The hostname, up to the first ‘.’.
                Color::Hostname => format!("H"),             // The hostname.
                Color::JobsCount => format!("j"),            // The number of jobs currently managed by the shell.
                Color::ShellDeviceName => format!("l"),      // The basename of the shell’s terminal device name.
                Color::Newline => format!("n"),              // A newline.
                Color::CarriageReturn => format!("r"),       // A carriage return.
                Color::ShellName => format!("s"), // The name of the shell, the basename of $0 (the portion following the final slash).
                Color::Time24hFormat => format!("t"), // The time, in 24-hour HH:MM:SS format.
                Color::Time12hFormat => format!("T"), // The time, in 12-hour HH:MM:SS format.
                Color::Time12hAmpm => format!("@"), // The time, in 12-hour am/pm format.
                Color::Time24hShort => format!("A"), // The time, in 24-hour HH:MM format.
                Color::Username => format!("u"),  // The username of the current user.
                Color::BashVersion => format!("v"), // The version of Bash (e.g., 2.00)
                Color::BashVersionFull => format!("V"), // The release of Bash, version + patchlevel (e.g., 2.00.0)
                Color::PwdShort => format!("w"), // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
                Color::PwdLong => format!("W"),  // The basename of $PWD, with $HOME abbreviated with a tilde.
                Color::HistoryNumber => format!("!"), // The history number of this command.
                Color::CommandNumber => format!("#"), // The command number of this command.
                Color::PromptEnd => format!("$"), // If the effective uid is 0, #, otherwise $.
                Color::Backslash => format!(r"\"), // A backslash.
                Color::BeginNonprinting => format!("["), // Begin a sequence of non-printing characters. This could be used to embed a terminal control sequence into the prompt.
                Color::EndNonprinting => format!("]"),   // End a sequence of non-printing characters.
            }
        )
    }
}
use crate::{Error, Result, Rule};

impl Node {
    fn u8_from_pair<'a>(pair: Pair<'a, Rule>) -> Result<u8> {
        Ok(u8::from_str_radix(pair.as_span().as_str(), 10)
            .map_err(|e| Error::ParseError(format!("{} (expected number from 0 to 255: {:#?})", e, pair.clone())))?)
    }

    fn u8_from_inner_pair<'a>(pair: Pair<'a, Rule>) -> Result<u8> {
        Self::u8_from_pair(pair.into_inner().next().expect("color"))
    }

    fn string_from_pair<'a>(pair: Pair<'a, Rule>) -> String {
        pair.as_span().as_str().to_string()
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Vec<Node>> {
        let mut tokens = Vec::<Node>::new();
        tokens.extend(match pair.as_rule() {
            Rule::node | Rule::ps1 | Rule::replacement => {
                let mut tokens = Vec::<Node>::new();
                for node in pair.clone().into_inner() {
                    tokens.extend(Node::from_pair(node)?);
                }
                tokens
            }
            Rule::unhandled => {
                vec![Node::Unhandled(pair.as_span().as_str().to_string())]
            }
            Rule::color => {
                vec![Node::Color(Self::u8_from_pair(pair)?)]
            }
            Rule::string => match pair.as_span().as_str() {
                "reset" => vec![Node::AnsiReset],
                string => {
                    vec![Node::Unhandled(format!("{{{}}}", string.to_string()))]
                }
            },
            Rule::reset => {
                vec![Node::AnsiReset]
            }
            Rule::fg_color => {
                vec![Node::Color(Self::u8_from_inner_pair(pair)?)]
            }
            Rule::bg_color => {
                vec![Node::BgColor(Self::u8_from_inner_pair(pair)?)]
            }
            Rule::vcs_param => {
                let mut pairs = pair.into_inner();

                let vcs = Self::string_from_pair(pairs.next().expect("vcs"));
                let param = Self::string_from_pair(pairs.next().expect("branch"));
                if vcs == "git" || vcs == "hg" {
                    vec![Node::VcsParam(vcs, param)]
                } else {
                    vec![Node::KeyValueParam(vcs, param)]
                }
            }
            Rule::key_value_param => {
                let mut pairs = pair.into_inner();
                let key = Self::string_from_pair(pairs.next().expect("key"));
                let value = Self::string_from_pair(pairs.next().expect("param"));
                vec![Node::KeyValueParam(key, value)]
            }

            Rule::vcs => {
                unreachable!("{:#?}", pair);
            }
            Rule::escape_variable | Rule::strftime_format => {
                // eprintln!("\n\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));
                // dbg!(&pairs);
                // eprintln!("\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));
                let mut pairs = pair.into_inner();
                let key = Self::string_from_pair(pairs.next().expect("key"));
                let value = Self::string_from_pair(pairs.next().expect("param"));
                vec![Node::Color(Color::Strftime(value))]
            }
            Rule::variable => {
                let pair = pair.into_inner().next().expect("variable");
                let variable = Color::from_pair(pair)?;
                vec![Node::Color(variable)]
            }
            Rule::variable_bell
            | Rule::variable_date_weekday
            | Rule::variable_strftime
            | Rule::variable_escape_character
            | Rule::variable_hostname_short
            | Rule::variable_hostname
            | Rule::variable_jobs_count
            | Rule::variable_shell_device_name
            | Rule::variable_newline
            | Rule::variable_carriage_return
            | Rule::variable_shell_name
            | Rule::variable_time_24h_format
            | Rule::variable_time_12h_format
            | Rule::variable_time_12h_ampm
            | Rule::variable_time_24h_short
            | Rule::variable_username
            | Rule::variable_bash_version
            | Rule::variable_bash_version_full
            | Rule::variable_pwd_short
            | Rule::variable_pwd_long
            | Rule::variable_history_number
            | Rule::variable_command_number
            | Rule::variable_prompt_end
            | Rule::variable_ascii_octal_code
            | Rule::variable_backslash
            | Rule::variable_begin_nonprinting
            | Rule::variable_end_nonprinting
            | Rule::variable_code_bell
            | Rule::variable_code_date_weekday
            | Rule::variable_code_strftime
            | Rule::variable_code_escape_character
            | Rule::variable_code_hostname_short
            | Rule::variable_code_hostname
            | Rule::variable_code_jobs_count
            | Rule::variable_code_shell_device_name
            | Rule::variable_code_newline
            | Rule::variable_code_carriage_return
            | Rule::variable_code_shell_name
            | Rule::variable_code_time_24h_format
            | Rule::variable_code_time_12h_format
            | Rule::variable_code_time_12h_ampm
            | Rule::variable_code_time_24h_short
            | Rule::variable_code_username
            | Rule::variable_code_bash_version
            | Rule::variable_code_bash_version_full
            | Rule::variable_code_pwd_short
            | Rule::variable_code_pwd_long
            | Rule::variable_code_history_number
            | Rule::variable_code_command_number
            | Rule::variable_code_prompt_end
            | Rule::variable_code_ascii_octal_code
            | Rule::variable_code_backslash
            | Rule::variable_code_begin_nonprinting
            | Rule::variable_code_end_nonprinting => {
                unreachable!("{:#?}", pair)
            }
            Rule::EOI | Rule::WHITESPACE => Vec::<Node>::new(),
        });
        Ok(tokens)
    }

    pub fn to_str(&self) -> String {
        match self.clone() {
            Node::AnsiReset => colors::wrap_np(colors::reset()),
            Node::Color(color) => colors::wrap_np(colors::fg(color)),
            Node::BgColor(color) => colors::wrap_np(colors::bg(color)),
            Node::Color(var) => var.to_string(),
            Node::VcsParam(vcs, param) => {
                format!("`$(ps1 --resolve {}:{})`", vcs, param)
            }
            Node::KeyValueParam(key, value) => {
                format!("`$(ps1 --resolve {}:{})`", key, value)
            }
            Node::Unhandled(string) => string.to_string(),
        }
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
// impl std::fmt::Debug for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:#?}", self.to_str())
//     }
// }
