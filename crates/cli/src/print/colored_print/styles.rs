// copied from termcolor
use super::{ColorChoice, DiffStyles};
use ansi_term::{Color, Style};
use anyhow::Result;
use std::env;

// warn[rule-id]: rule message here.
// |------------|------------------|
//    header            message
#[derive(Default)]
pub struct RuleStyle {
  // header style
  pub error: Style,
  pub warning: Style,
  pub info: Style,
  pub hint: Style,
  // message style
  pub message: Style,
  pub note: Style,
}

impl RuleStyle {
  fn colored() -> Self {
    Self {
      error: Color::Red.bold(),
      warning: Color::Yellow.bold(),
      info: Style::new().bold(),
      hint: Style::new().dimmed().bold(),
      note: Style::new().italic(),
      message: Style::new().bold(),
    }
  }
}

// TODO: use termcolor instead
#[derive(Default)]
pub struct PrintStyles {
  // print match color
  pub file_path: Style,
  pub matched: Style,
  pub rule: RuleStyle,
  pub diff: DiffStyles,
}

impl PrintStyles {
  fn colored() -> Self {
    Self {
      file_path: Color::Cyan.italic(),
      matched: Color::Red.bold(),
      diff: DiffStyles::colored(),
      rule: RuleStyle::colored(),
    }
  }
  fn no_color() -> Self {
    Self::default()
  }

  pub fn push_matched_to_ret(&self, ret: &mut String, matched: &str) -> Result<()> {
    use std::fmt::Write;
    // TODO: use intersperse
    let mut lines = matched.lines();
    if let Some(line) = lines.next() {
      write!(ret, "{}", self.matched.paint(line))?;
    } else {
      return Ok(());
    }
    for line in lines {
      ret.push('\n');
      write!(ret, "{}", self.matched.paint(line))?;
    }
    Ok(())
  }
}
impl From<ColorChoice> for PrintStyles {
  fn from(color: ColorChoice) -> Self {
    if should_use_color(&color) {
      Self::colored()
    } else {
      Self::no_color()
    }
  }
}

/// Returns true if we should attempt to write colored output.
pub fn should_use_color(color: &ColorChoice) -> bool {
  match *color {
    // TODO: we should check if ansi is supported on windows console
    ColorChoice::Always => true,
    ColorChoice::AlwaysAnsi => true,
    ColorChoice::Never => false,
    // NOTE tty check is added
    ColorChoice::Auto => atty::is(atty::Stream::Stdout) && env_allows_color(),
  }
}

fn env_allows_color() -> bool {
  match env::var_os("TERM") {
    // On Windows, if TERM isn't set, then we should not automatically
    // assume that colors aren't allowed. This is unlike Unix environments
    None => {
      if !cfg!(windows) {
        return false;
      }
    }
    Some(k) => {
      if k == "dumb" {
        return false;
      }
    }
  }
  // If TERM != dumb, then the only way we don't allow colors at this
  // point is if NO_COLOR is set.
  if env::var_os("NO_COLOR").is_some() {
    return false;
  }
  true
}
