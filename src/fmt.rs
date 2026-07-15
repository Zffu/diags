use std::{cmp::min, fmt::Display, fs, path::PathBuf};

use colored::Colorize;

use crate::{
    Diagnostic, DiagnosticCode, Level,
    pos::PositionDelimiter,
    span::{LabelledSpan, MainSpan, SecondarySpan},
};

pub const PRIMARY_SPAN_CHAR: char = '^';
pub const SECONDARY_SPAN_CHAR: char = '#';

pub fn do_span_format(
    span: &LabelledSpan,
    f: &mut std::fmt::Formatter<'_>,
    span_char: char,
) -> std::fmt::Result {
    let lines = match get_line_between_positions(
        PositionDelimiter::new(span.span.start_line, span.span.start_col),
        PositionDelimiter::new(span.span.end_line, span.span.end_col),
        &span.span.file,
    ) {
        Ok(v) => v,
        Err(err) => {
            write!(f, "Cannot read file: {}", err)?;
            return Ok(());
        }
    };

    let mut line_index = span.span.start_line;

    for line in lines {
        writeln!(f, "    {}    {}", "|".bright_magenta(), line)?;

        let underline;

        if span.span.start_line == span.span.end_line {
            underline = print_line(span.span.start_col, span.span.end_col, span_char);
        } else if line_index == span.span.start_line {
            underline = print_line(span.span.start_col, line.len(), span_char);
        } else if line_index == span.span.end_line {
            underline = print_line(0, span.span.end_col, span_char);
        } else {
            underline = print_line(0, line.len(), span_char);
        }

        writeln!(
            f,
            "    {}    {}",
            "|".bright_magenta(),
            underline.bright_yellow()
        )?;

        line_index += 1;
    }

    if let Some(label) = &span.label {
        let space = print_blank_line(span.span.start_col);
        writeln!(f, "    {}    {}{}", "|".bright_magenta(), space, label)?;
    }

    Ok(())
}

impl Display for MainSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        do_span_format(&self.0, f, PRIMARY_SPAN_CHAR)
    }
}

impl Display for SecondarySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        do_span_format(&self.0, f, SECONDARY_SPAN_CHAR)
    }
}

impl Display for DiagnosticCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = match self.level {
            Level::Error => "E",
            Level::CriticalError => "E",
            Level::Warning => "W",
            Level::Info => "I",
        }
        .to_string();

        str.push_str(&format!("{}", self.code));

        let mut str = match self.level {
            Level::Error => str.red(),
            Level::CriticalError => str.bright_red(),
            Level::Warning => str.bright_yellow(),
            Level::Info => str.bright_green(),
        };

        str = str.bold();

        write!(f, "{}", str)
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}]: {}", self.code, self.message)?;

        if let Some(primary_span) = self.primary_span.clone() {
            writeln!(
                f,
                "  {} {:#?}:{}:{}",
                "-->".bright_magenta(),
                primary_span.span.file,
                primary_span.span.start_line,
                primary_span.span.start_col
            )?;
            writeln!(f, "   {}", "|".bright_magenta())?;

            write!(f, "{}", MainSpan(primary_span))?;
        }

        for span in self.spans.clone() {
            writeln!(f, "{}", SecondarySpan(span))?;
        }

        writeln!(f, "   {}", "|".bright_magenta())?;

        let mut ind = 0;

        for note in &self.notes {
            writeln!(f, "   {} {}: {}", "=".bright_blue(), "note".bold(), note)?;
            writeln!(
                f,
                "   {} {}: {}",
                "=".bright_blue(),
                "help".bold(),
                self.helps[ind]
            )?;
            write!(f, "   {}", "|".bright_magenta())?;

            ind += 1;
        }

        Ok(())
    }
}

fn get_line_between_positions(
    start: PositionDelimiter,
    end: PositionDelimiter,
    file: &PathBuf,
) -> Result<Vec<String>, std::io::Error> {
    let lines: Vec<String> = fs::read_to_string(file)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    if start.line == end.line {
        Ok(vec![lines[start.line].clone()])
    } else {
        Ok(lines[start.line..min(lines.len() - 1, end.line + 1)].to_vec())
    }
}

fn print_line(start: usize, end: usize, c: char) -> String {
    let mut str = String::new();

    for _ in 0..start {
        str.push(' ');
    }

    for _ in start..end {
        str.push(c);
    }

    str
}

fn print_blank_line(end: usize) -> String {
    let mut str = String::new();

    for _ in 0..end {
        str.push(' ');
    }

    str
}
