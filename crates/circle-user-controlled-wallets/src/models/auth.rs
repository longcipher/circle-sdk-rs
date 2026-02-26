//! Authentication models for the Circle User-Controlled Wallets API.
//!
//! Contains request and response types for device-token, email OTP, and
//! user-token refresh flows.

use serde::{Deserialize, Serialize};

// ── Social device token ───────────────────────────────────────────────────────

/// `data` payload for a social sign-in device-token response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenSocialData {
    /// Short-lived device token for the Circle mobile SDK.
    pub device_token: String,
    /// Encryption key paired with the device token.
    pub device_encryption_key: Option<String>,
}

/// Response envelope for `getDeviceTokenSocial`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenSocialResponse {
    /// Device token data.
    pub data: DeviceTokenSocialData,
}

// ── Email device token ────────────────────────────────────────────────────────

/// `data` payload for an email sign-in device-token response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenEmailData {
    /// Short-lived device token for the Circle mobile SDK.
    pub device_token: String,
    /// Encryption key paired with the device token.
    pub device_encryption_key: Option<String>,
    /// OTP token to be verified by the user.
    pub otp_token: Option<String>,
}

/// Response envelope for `getDeviceTokenEmail`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenEmailResponse {
    /// Email device token data.
    pub data: DeviceTokenEmailData,
}

// ── Refresh user token ────────────────────────────────────────────────────────

/// `data` payload for a refreshed user token.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshUserTokenData {
    /// New short-lived user JWT.
    pub user_token: String,
    /// New encryption key.
    pub encryption_key: Option<String>,
    /// ID of the authenticated user.
    pub user_id: Option<String>,
    /// Opaque refresh token for the next refresh cycle.
    pub refresh_token: Option<String>,
}

/// Response envelope for `refreshUserToken`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshUserTokenResponse {
    /// Refreshed token data.
    pub data: RefreshUserTokenData,
}

// ── Resend OTP ────────────────────────────────────────────────────────────────

/// `data` payload for a resend-OTP response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendOtpData {
    /// New OTP token issued to the user.
    pub otp_token: String,
}

/// Response envelope for `resendOtp`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendOtpResponse {
    /// OTP data.
    pub data: ResendOtpData,
}

// ── Request bodies ────────────────────────────────────────────────────────────

/// Request body for `getDeviceTokenSocial`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenSocialRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Unique identifier for the user's device.
    pub device_id: String,
}

/// Request body for `getDeviceTokenEmail`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenEmailRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Unique identifier for the user's device.
    pub device_id: String,
    /// Email address for OTP delivery.
    pub email: String,
}

/// Request body for `refreshUserToken`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshUserTokenRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Refresh token from a previous authentication.
    pub refresh_token: String,
    /// Unique identifier for the user's device.
    pub device_id: String,
}

/// Request body for `resendOtp`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendOtpRequest {
    /// Client-generated idempotency key (UUID).
    pub idempotency_key: String,
    /// Active OTP token to be superseded.
    pub otp_token: String,
    /// Email address for OTP delivery.
    pub email: String,
    /// Unique identifier for the user's device.
    pub device_id: String,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_token_social_request_camel_case() -> Result<(), Box<dyn std::error::Error>> {
        let req = DeviceTokenSocialRequest {
            idempotency_key: "key1".to_string(),
            device_id: "dev1".to_string(),
        };
        let s = serde_json::to_string(&req)?;
        assert!(s.contains("idempotencyKey"), "{s}");
        assert!(s.contains("deviceId"), "{s}");
        Ok(())
    }

    #[test]
    fn refresh_user_token_response_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"{"data":{"userToken":"tok","encryptionKey":"key","userId":"u1","refreshToken":"rt"}}"#;
        let resp: RefreshUserTokenResponse = serde_json::from_str(json)?;
        assert_eq!(resp.data.user_token, "tok");
        assert_eq!(resp.data.user_id.as_deref(), Some("u1"));
        Ok(())
    }
}
