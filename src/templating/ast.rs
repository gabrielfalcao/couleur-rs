use chrono::Utc;
use pest::iterators::Pair;

use crate::color as colors;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    AnsiReset,
    Color(u8),
    BgColor(u8),
    VcsParam(String, String),
    KeyValueParam(String, String),
    Unhandled(String),
    Variable(Variable),
}

impl Token {
    pub fn default_vec() -> Vec<Token> {
        vec![
            Token::Color(79),
            Token::Variable(Variable::Username),
            Token::Unhandled("@".to_string()),
            Token::Variable(Variable::PwdShort),
            Token::Variable(Variable::PromptEnd),
            Token::AnsiReset,
        ]
    }
}
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Variable {
    AsciiOctalCode(u8),                // `\nnn`        // The character whose ASCII code is the octal value nnn.
    Strftime(String),                  // `\D{format}`  // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
    Bell,                              // `\a`          // A bell character.
    DateWeekday,                       // `\d`          // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
    EscapeCharacter,                   // `\e`          // An escape character.
    HostnameShort,                     // `\h`          // The hostname, up to the first ‘.’.
    Hostname,                          // `\H`          // The hostname.
    JobsCount,                         // `\j`          // The number of jobs currently managed by the shell.
    ShellDeviceName,                   // `\l`          // The basename of the shell’s terminal device name.
    Newline,                           // `\n`          // A newline.
    CarriageReturn,                    // `\r`          // A carriage return.
    ShellName,                         // `\s`          // The name of the shell, the basename of $0 (the portion following the final slash).
    Time24hFormat,                     // `\t`          // The time, in 24-hour HH:MM:SS format.
    Time12hFormat,                     // `\T`          // The time, in 12-hour HH:MM:SS format.
    Time12hAmpm,                       // `\@`          // The time, in 12-hour am/pm format.
    Time24hShort,                      // `\A`          // The time, in 24-hour HH:MM format.
    Username,                          // `\u`          // The username of the current user.
    BashVersion,                       // `\v`          // The version of Bash (e.g., 2.00)
    BashVersionFull,                   // `\V`          // The release of Bash, version + patchlevel (e.g., 2.00.0)
    PwdShort,                          // `\w`          // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
    PwdLong,                           // `\W`          // The basename of $PWD, with $HOME abbreviated with a tilde.
    HistoryNumber,                     // `\!`          // The history number of this command.
    CommandNumber,                     // `\#`          // The command number of this command.
    PromptEnd,                         // `\$`          // If the effective uid is 0, #, otherwise $.
    Backslash,                         // `\\`          // A backslash.
    BeginNonprinting,                  // `\[`          // Begin a sequence of non-printing characters. This could be used to embed a terminal control sequence into the prompt.
    EndNonprinting,                    // `\]`          // End a sequence of non-printing characters.
}
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[rustfmt::skip]
impl Variable {
    pub fn to_str(&self) -> String {
        let now = Utc::now();
        let hostname = iocore::shell_command_string_output("hostname", ".")
            .map(|(_, hostname, _)| hostname)
            .unwrap_or_default();

        match self {
            Variable::Bell => format!(r"\a"), // A bell character.
            Variable::AsciiOctalCode(c) => format!(r"\{:o}", c),
            Variable::Strftime(f) => format!(r"\D{{{}}}", f),          // {format}` // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
            Variable::DateWeekday => format!(r"\d"), // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
            Variable::EscapeCharacter => format!(r"\e"), // An escape character.
            Variable::HostnameShort => format!(r"\H"), // The hostname, up to the first ‘.’.
            Variable::Hostname => format!(r"\h"), // The hostname.
            Variable::JobsCount => format!(r"\j"), // The number of jobs currently managed by the shell.
            Variable::ShellDeviceName => format!(r"\l"), // The basename of the shell’s terminal device name.
            Variable::Newline => format!(r"\n"), // A newline.
            Variable::CarriageReturn => format!(r"\r"), // A carriage return.
            Variable::ShellName => format!(r"\s"), // The name of the shell, the basename of $0 (the portion following the final slash).
            Variable::Time24hFormat => format!(r"\t"), // The time, in 24-hour HH:MM:SS format.
            Variable::Time12hFormat => format!(r"\T"), // The time, in 12-hour HH:MM:SS format.
            Variable::Time12hAmpm => format!(r"\@"), // The time, in 12-hour am/pm format.
            Variable::Time24hShort => format!(r"\A"), // The time, in 24-hour HH:MM format.
            Variable::Username => format!(r"\u"), // The username of the current user.
            Variable::BashVersion => format!(r"\v"), // The version of Bash (e.g., 2.00)
            Variable::BashVersionFull => format!(r"\V"), // The release of Bash, version + patchlevel (e.g., 2.00.0)
            Variable::PwdShort => format!(r"\W"), // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
            Variable::PwdLong => format!(r"\w"), // The basename of $PWD, with $HOME abbreviated with a tilde.
            Variable::HistoryNumber => format!(r"\!"), // The history number of this command.
            Variable::CommandNumber => format!(r"\#"), // The command number of this command.
            Variable::PromptEnd => format!(r"\$"),
            Variable::Backslash => format!(r"\\"), // A backslash.
            Variable::BeginNonprinting => format!(r"\["), // Begin a sequence of non-printing characters. This could be used to embed a terminal control sequence into the prompt.
            Variable::EndNonprinting => format!(r"\]"), // End a sequence of non-printing characters.
        }
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Variable> {
        let pair = pair.clone();
        let rule = pair.clone().as_rule();
        let mut pairs = pair.clone().into_inner();

        pairs.next().unwrap(); // consume "escape_character"
        let variable = match &rule {
            Rule::variable_strftime => {
                Variable::Strftime(
                    pairs
                        .next()
                        .expect("strftime")
                        .as_span()
                        .as_str()
                        .to_string(),
                ) // `\D{format}` // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
            },
            Rule::variable_ascii_octal_code => {
                let value = pairs
                    .next()
                    .expect("octal")
                    .as_span()
                    .as_str()
                    .to_string();
                let code = u8::from_str_radix(value.as_str(), 8).expect("octal");
                Variable::AsciiOctalCode(code) // `\nnn` // The character whose ASCII code is the octal value nnn.
            },
            Rule::variable_bell => {
                Variable::Bell // `\a` // A bell character.
            },
            Rule::variable_date_weekday => {
                Variable::DateWeekday // `\d` // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
            },

            Rule::variable_escape_character => {
                Variable::EscapeCharacter // `\e` // An escape character.
            },
            Rule::variable_hostname_short => {
                Variable::HostnameShort // `\h` // The hostname, up to the first ‘.’.
            },
            Rule::variable_hostname => {
                Variable::Hostname // `\H` // The hostname.
            },
            Rule::variable_jobs_count => {
                Variable::JobsCount // `\j` // The number of jobs currently managed by the shell.
            },
            Rule::variable_shell_device_name => {
                Variable::ShellDeviceName // `\l` // The basename of the shell’s terminal device name.
            },
            Rule::variable_newline => {
                Variable::Newline // `\n` // A newline.
            },
            Rule::variable_carriage_return => {
                Variable::CarriageReturn // `\r` // A carriage return.
            },
            Rule::variable_shell_name => {
                Variable::ShellName // `\s` // The name of the shell, the basename of $0 (the portion following the final slash).
            },
            Rule::variable_time_24h_format => {
                Variable::Time24hFormat // `\t` // The time, in 24-hour HH:MM:SS format.
            },
            Rule::variable_time_12h_format => {
                Variable::Time12hFormat // `\T` // The time, in 12-hour HH:MM:SS format.
            },
            Rule::variable_time_12h_ampm => {
                Variable::Time12hAmpm // `\@` // The time, in 12-hour am/pm format.
            },
            Rule::variable_time_24h_short => {
                Variable::Time24hShort // `\A` // The time, in 24-hour HH:MM format.
            },
            Rule::variable_username => {
                Variable::Username // `\u` // The username of the current user.
            },
            Rule::variable_bash_version => {
                Variable::BashVersion // `\v` // The version of Bash (e.g., 2.00)
            },
            Rule::variable_bash_version_full => {
                Variable::BashVersionFull // `\V` // The release of Bash, version + patchlevel (e.g., 2.00.0)
            },
            Rule::variable_pwd_short => {
                Variable::PwdShort // `\w` // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
            },
            Rule::variable_pwd_long => {
                Variable::PwdLong // `\W` // The basename of $PWD, with $HOME abbreviated with a tilde.
            },
            Rule::variable_history_number => {
                Variable::HistoryNumber // `\!` // The history number of this command.
            },
            Rule::variable_command_number => {
                Variable::CommandNumber // `\#` // The command number of this command.
            },
            Rule::variable_prompt_end => {
                Variable::PromptEnd // `\$` // If the effective uid is 0, #, otherwise $.
            },
            Rule::variable_backslash => {
                Variable::Backslash // `\\` // A backslash.
            },
            Rule::variable_begin_nonprinting => {
                Variable::BeginNonprinting // `\[` // Begin a sequence of non-printing characters.
            },
            Rule::variable_end_nonprinting => {
                Variable::EndNonprinting // `\]` // End a sequence of non-printing characters.
            },
            _ => {
                unreachable!("{:#?}", pair);
            },
        };
        Ok(variable)
    }

    pub fn repr(&self) -> String {
        format!(
            "\\{}",
            match self {
                Variable::AsciiOctalCode(c) => format!("{:03o}", c),   // The character whose ASCII code is the octal value nnn.
                Variable::Strftime(f) => format!("D{{{}}}", f),        // The format is passed to strftime(3) and the result is inserted into the prompt string; an empty format results in a locale-specific time representation. The braces are required. ~ "{" ~ strftime_format ~ "}"
                Variable::Bell => format!("a"),                        // A bell character.
                Variable::DateWeekday => format!("d"),                 // The date, in "Weekday Month Date" format (e.g., "Tue May 26").
                Variable::EscapeCharacter => format!("e"),             // An escape character.
                Variable::HostnameShort => format!("h"),               // The hostname, up to the first ‘.’.
                Variable::Hostname => format!("H"),                    // The hostname.
                Variable::JobsCount => format!("j"),                   // The number of jobs currently managed by the shell.
                Variable::ShellDeviceName => format!("l"),             // The basename of the shell’s terminal device name.
                Variable::Newline => format!("n"),                     // A newline.
                Variable::CarriageReturn => format!("r"),              // A carriage return.
                Variable::ShellName => format!("s"),                   // The name of the shell, the basename of $0 (the portion following the final slash).
                Variable::Time24hFormat => format!("t"),               // The time, in 24-hour HH:MM:SS format.
                Variable::Time12hFormat => format!("T"),               // The time, in 12-hour HH:MM:SS format.
                Variable::Time12hAmpm => format!("@"),                 // The time, in 12-hour am/pm format.
                Variable::Time24hShort => format!("A"),                // The time, in 24-hour HH:MM format.
                Variable::Username => format!("u"),                    // The username of the current user.
                Variable::BashVersion => format!("v"),                 // The version of Bash (e.g., 2.00)
                Variable::BashVersionFull => format!("V"),             // The release of Bash, version + patchlevel (e.g., 2.00.0)
                Variable::PwdShort => format!("w"),                    // The value of the PWD shell variable ($PWD), with $HOME abbreviated with a tilde (uses the $PROMPT_DIRTRIM variable).
                Variable::PwdLong => format!("W"),                     // The basename of $PWD, with $HOME abbreviated with a tilde.
                Variable::HistoryNumber => format!("!"),               // The history number of this command.
                Variable::CommandNumber => format!("#"),               // The command number of this command.
                Variable::PromptEnd => format!("$"),                   // If the effective uid is 0, #, otherwise $.
                Variable::Backslash => format!(r"\"),                  // A backslash.
                Variable::BeginNonprinting => format!("["),            // Begin a sequence of non-printing characters. This could be used to embed a terminal control sequence into the prompt.
                Variable::EndNonprinting => format!("]"),              // End a sequence of non-printing characters.
            }
        )
    }
}
use crate::{Error, Result, Rule};

impl Token {
    fn u8_from_pair<'a>(pair: Pair<'a, Rule>) -> Result<u8> {
        Ok(u8::from_str_radix(pair.as_span().as_str(), 10).map_err(|e| {
            Error::ParseError(format!(
                "{} (expected number from 0 to 255: {:#?})",
                e,
                pair.clone()
            ))
        })?)
    }

    fn u8_from_inner_pair<'a>(pair: Pair<'a, Rule>) -> Result<u8> {
        Self::u8_from_pair(pair.into_inner().next().expect("color"))
    }

    fn string_from_pair<'a>(pair: Pair<'a, Rule>) -> String {
        pair.as_span().as_str().to_string()
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();
        tokens.extend(match pair.as_rule() {
            Rule::token | Rule::ps1 | Rule::replacement => {
                let mut tokens = Vec::<Token>::new();
                for token in pair.clone().into_inner() {
                    tokens.extend(Token::from_pair(token)?);
                }
                tokens
            },
            Rule::unhandled => {
                vec![Token::Unhandled(
                    pair.as_span().as_str().to_string(),
                )]
            },
            Rule::color => {
                vec![Token::Color(Self::u8_from_pair(pair)?)]
            },
            Rule::string => match pair.as_span().as_str() {
                "reset" => vec![Token::AnsiReset],
                string => {
                    vec![Token::Unhandled(format!(
                        "{{{}}}",
                        string.to_string()
                    ))]
                },
            },
            Rule::reset => {
                vec![Token::AnsiReset]
            },
            Rule::fg_color => {
                vec![Token::Color(Self::u8_from_inner_pair(
                    pair,
                )?)]
            },
            Rule::bg_color => {
                vec![Token::BgColor(Self::u8_from_inner_pair(
                    pair,
                )?)]
            },
            Rule::vcs_param => {
                let mut pairs = pair.into_inner();

                let vcs = Self::string_from_pair(pairs.next().expect("vcs"));
                let param = Self::string_from_pair(pairs.next().expect("branch"));
                if vcs == "git" || vcs == "hg" {
                    vec![Token::VcsParam(vcs, param)]
                } else {
                    vec![Token::KeyValueParam(vcs, param)]
                }
            },
            Rule::key_value_param => {
                let mut pairs = pair.into_inner();
                let key = Self::string_from_pair(pairs.next().expect("key"));
                let value = Self::string_from_pair(pairs.next().expect("param"));
                vec![Token::KeyValueParam(key, value)]
            },

            Rule::vcs => {
                unreachable!("{:#?}", pair);
            },
            Rule::escape_variable | Rule::strftime_format => {
                // eprintln!("\n\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));
                // dbg!(&pairs);
                // eprintln!("\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));
                let mut pairs = pair.into_inner();
                let key = Self::string_from_pair(pairs.next().expect("key"));
                let value = Self::string_from_pair(pairs.next().expect("param"));
                vec![Token::Variable(Variable::Strftime(value))]
            },
            Rule::variable => {
                let pair = pair.into_inner().next().expect("variable");
                let variable = Variable::from_pair(pair)?;
                vec![Token::Variable(variable)]
            },
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
            },
            Rule::EOI | Rule::WHITESPACE => Vec::<Token>::new(),
        });
        Ok(tokens)
    }

    pub fn to_str(&self) -> String {
        match self.clone() {
            Token::AnsiReset => colors::wrap_np(colors::reset()),
            Token::Color(color) => colors::wrap_np(colors::fg(color)),
            Token::BgColor(color) => colors::wrap_np(colors::bg(color)),
            Token::Variable(var) => var.to_string(),
            Token::VcsParam(vcs, param) => {
                format!("`$(ps1 --resolve {}:{})`", vcs, param)
            },
            Token::KeyValueParam(key, value) => {
                format!("`$(ps1 --resolve {}:{})`", key, value)
            },
            Token::Unhandled(string) => string.to_string(),
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
// impl std::fmt::Debug for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:#?}", self.to_str())
//     }
// }
