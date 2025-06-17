// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // 1. Decode the hex string
    let raw_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // 2. Ensure there are at least 4 bytes
    if raw_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // 3. Read the first 4 bytes as little-endian u32
    let _version_bytes: [u8; 4] = raw_bytes[0..4]
        .try_into()
        .map_err(|_| "Failed to extract version bytes".to_string())?;

    Ok(u32::from_le_bytes([
        raw_bytes[0],
        raw_bytes[1],
        raw_bytes[2],
        raw_bytes[3],
    ]))
}
