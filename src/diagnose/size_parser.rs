use crate::PgExtrasError;

/// Parses the index size string to a `u64` value.
///
/// # Arguments
/// * `size` - A string slice representing the size (e.g., "80 kB", "18 MB").
///
/// # Returns
/// A `Result` containing the size in bytes as `u64` if successful, or a `PgExtrasError` if an error occurs.
pub fn to_bytes(size: &str) -> Result<u64, PgExtrasError> {
    let size = size.trim();
    let (num, unit) = size.split_at(size.find(char::is_alphabetic).unwrap_or(size.len()));
    let num: f64 = num.trim().parse().map_err(|_| PgExtrasError::Unknown(size.to_string()))?;
    let multiplier: i64 = match unit.trim().to_lowercase().as_str() {
        "b" => 1,
        "kb" => 1_000,
        "mb" => 1_000_000,
        "gb" => 1_000_000_000,
        "tb" => 1_000_000_000_000,
        _ => return Err(PgExtrasError::Unknown(size.to_string())),
    };
    Ok((num * multiplier as f64) as u64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_index_size_kb() {
        let result = to_bytes("80 kB").unwrap();
        assert_eq!(result, 80_000);
    }

    #[test]
    fn parse_index_size_mb() {
        let result = to_bytes("18 MB").unwrap();
        assert_eq!(result, 18_000_000);
    }

    #[test]
    fn parse_index_size_gb() {
        let result = to_bytes("2 GB").unwrap();
        assert_eq!(result, 2_000_000_000);
    }

    #[test]
    fn parse_index_size_tb() {
        let result = to_bytes("1 TB").unwrap();
        assert_eq!(result, 1_000_000_000_000);
    }

    #[test]
    fn parse_index_size_bytes() {
        let result = to_bytes("500 B").unwrap();
        assert_eq!(result, 500);
    }

    #[test]
    fn parse_index_size_invalid_format() {
        let result = to_bytes("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn parse_index_size_unknown_unit() {
        let result = to_bytes("100 XY");
        assert!(result.is_err());
    }
}