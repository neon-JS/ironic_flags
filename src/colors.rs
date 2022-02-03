/// Convert rgb color (range 0 - 255) to ansi color code
///
/// # Arguments
///
/// * `r` - red value
/// * `g` - green value
/// * `b` - blue value
///
/// # Examples
/// ```
/// let ansi_code_red = rgb(255, 0, 0);
/// print!("{}Some red text!", ansi_color_red);
/// ```
pub fn rgb(r: u8, g: u8, b: u8) -> String
{
    let range_factor = 6.0 / 256.0;

    let ansi_terminal_color_code = 16.0 + 36.0 * (r as f32 * range_factor).floor()
        + 6.0 * (g as f32 * range_factor).floor()
        + (b as f32 * range_factor).floor();

    format!("\x1b[48;5;{}m", ansi_terminal_color_code)
}

/// Returns ansi reset code
///
/// # Examples
/// ```
/// let ansi_code_red = rgb(255, 0, 0);
/// print!("{}Some red text! {}Some default text!", ansi_color_red, reset());
/// ```
pub fn reset() -> &'static str
{
    "\x1b[0m"
}
