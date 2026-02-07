/// Color parsing module
/// Supports HEX, RGB, and RGBL color formats

use anyhow::{anyhow, Result};

pub fn parse_color(input: &str) -> Result<(u8, u8, u8)> {
    let input = input.trim();
    
    if input.is_empty() {
        return Err(anyhow!("Empty input"));
    }
    
    // Try HEX format
    if input.starts_with('#') {
        return parse_hex(input);
    }
    
    // Try RGB or RGBL format
    if input.contains(',') {
        let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
        
        if parts.len() == 3 {
            return parse_rgb(&parts);
        } else if parts.len() == 4 {
            return parse_rgbl(&parts);
        }
    }
    
    // Try HEX without #
    if input.len() == 6 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        return parse_hex(&format!("#{}", input));
    }
    
    Err(anyhow!("Invalid color format. Use HEX (#FF0000), RGB (255,0,0), or RGBL format"))
}

fn parse_hex(hex: &str) -> Result<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err(anyhow!("HEX color must be 6 characters (e.g., #FF0000)"));
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| anyhow!("Invalid HEX value for red channel"))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| anyhow!("Invalid HEX value for green channel"))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| anyhow!("Invalid HEX value for blue channel"))?;
    
    Ok((r, g, b))
}

fn parse_rgb(parts: &[&str]) -> Result<(u8, u8, u8)> {
    if parts.len() != 3 {
        return Err(anyhow!("RGB format requires exactly 3 values"));
    }
    
    let r = parse_channel(parts[0], "red")?;
    let g = parse_channel(parts[1], "green")?;
    let b = parse_channel(parts[2], "blue")?;
    
    Ok((r, g, b))
}

fn parse_rgbl(parts: &[&str]) -> Result<(u8, u8, u8)> {
    if parts.len() != 4 {
        return Err(anyhow!("RGBL format requires exactly 4 values"));
    }
    
    let r = parse_channel(parts[0], "red")?;
    let g = parse_channel(parts[1], "green")?;
    let b = parse_channel(parts[2], "blue")?;
    // Luminance (4th value) is ignored for this application
    
    Ok((r, g, b))
}

fn parse_channel(value: &str, channel_name: &str) -> Result<u8> {
    let value = value.trim();
    
    // Try to parse as integer
    value.parse::<u8>()
        .map_err(|_| anyhow!("Invalid {} channel value: '{}'. Must be 0-255", channel_name, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_with_hash() {
        assert_eq!(parse_color("#FF0000").unwrap(), (255, 0, 0));
        assert_eq!(parse_color("#00FF00").unwrap(), (0, 255, 0));
        assert_eq!(parse_color("#0000FF").unwrap(), (0, 0, 255));
    }

    #[test]
    fn test_parse_hex_without_hash() {
        assert_eq!(parse_color("FF0000").unwrap(), (255, 0, 0));
    }

    #[test]
    fn test_parse_rgb() {
        assert_eq!(parse_color("255,0,0").unwrap(), (255, 0, 0));
        assert_eq!(parse_color("0, 255, 0").unwrap(), (0, 255, 0));
        assert_eq!(parse_color("0,0,255").unwrap(), (0, 0, 255));
    }

    #[test]
    fn test_parse_rgbl() {
        assert_eq!(parse_color("255,0,0,128").unwrap(), (255, 0, 0));
    }

    #[test]
    fn test_invalid_formats() {
        assert!(parse_color("").is_err());
        assert!(parse_color("#GG0000").is_err());
        assert!(parse_color("256,0,0").is_err());
    }
}
