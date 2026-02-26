//! User resource models for the Circle User-Controlled Wallets API.
//!
//! Contains request parameters and response types for user registration and
//! management endpoints.

use serde::{Deserialize, Serialize};

use super::common::PageParams;

// ── Enums ─────────────────────────────────────────────────────────────────────

/// PIN status for an end-user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PinStatus {
    /// PIN has been set and is active.
    Enabled,
    /// PIN has not been configured yet.
    Unset,
    /// PIN is locked after too many failed attempts.
    Locked,
}

/// Overall account status of an end-user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndUserStatus {
    /// Account is active.
    Enabled,
    /// Account has been disabled.
    Disabled,
}

/// Status of security question recovery for an end-user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityQuestionStatus {
    /// Security questions are configured.
    Enabled,
    /// Security questions have not been set.
    Unset,
    /// Security question recovery is locked.
    Locked,
}

// ── Models ────────────────────────────────────────────────────────────────────

/// PIN security details for an end-user.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinSecurityDetails {
    /// Number of failed PIN attempts since last successful authentication.
    pub failed_attempts: Option<i32>,
    /// Date the PIN was locked.
    pub locked_date: Option<String>,
    /// Date the PIN lock expires.
    pub locked_expiry_date: Option<String>,
    /// Date of the last lock override by an admin.
    pub last_lock_override_date: Option<String>,
}

/// An end-user record returned by the Circle API.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndUser {
    /// Unique identifier for the end-user.
    pub id: Option<String>,
    /// ISO 8601 timestamp when the user was created.
    pub create_date: Option<String>,
    /// Current PIN status.
    pub pin_status: Option<PinStatus>,
    /// Current account status.
    pub status: Option<EndUserStatus>,
    /// Current security question status.
    pub security_question_status: Option<SecurityQuestionStatus>,
    /// Extended PIN security details (opaque JSON).
    pub pin_details: Option<serde_json::Value>,
    /// Extended security question details (opaque JSON).
    pub security_question_details: Option<serde_json::Value>,
}

// ── Response wrappers ─────────────────────────────────────────────────────────

/// `data` payload for list-users responses.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersData {
    /// List of end-users.
    pub users: Vec<EndUser>,
}

/// Response envelope for list-users.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    /// Paginated list of end-users.
    pub data: UsersData,
}

/// Response envelope for the `getUserByToken` endpoint.
///
/// The `data` field is the [`EndUser`] directly.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// The end-user record.
    pub data: EndUser,
}

/// Inner `data` payload for `getUserById`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdData {
    /// The end-user record.
    pub user: EndUser,
}

/// Response envelope for `getUser` (by ID).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdResponse {
    /// Wrapper containing the user.
    pub data: GetUserByIdData,
}

/// Payload inside a `userToken` response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenData {
    /// Short-lived JWT for the end-user session.
    pub user_token: String,
    /// Encryption key associated with the token.
    pub encryption_key: Option<String>,
}

/// Response envelope for `getUserToken`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenResponse {
    /// Token data payload.
    pub data: UserTokenData,
}

// ── Request bodies ────────────────────────────────────────────────────────────

/// Request body for `createUser`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    /// Application-defined identifier for the user.
    pub user_id: String,
}

/// Request body for `getUserToken`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserTokenRequest {
    /// The application user ID whose token should be retrieved.
    pub user_id: String,
}

/// Query parameters for `listUsers`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersParams {
    /// Filter by PIN status.
    pub pin_status: Option<PinStatus>,
    /// Pagination cursors.
    #[serde(flatten)]
    pub page: PageParams,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_status_screaming_snake_case() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::to_string(&PinStatus::Locked)?, "\"LOCKED\"");
        assert_eq!(serde_json::to_string(&PinStatus::Unset)?, "\"UNSET\"");
        Ok(())
    }

    #[test]
    fn end_user_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let user = EndUser {
            id: Some("user-123".to_string()),
            create_date: Some("2024-01-01T00:00:00Z".to_string()),
            pin_status: Some(PinStatus::Enabled),
            status: Some(EndUserStatus::Enabled),
            security_question_status: Some(SecurityQuestionStatus::Unset),
            pin_details: None,
            security_question_details: None,
        };
        let json = serde_json::to_string(&user)?;
        let decoded: EndUser = serde_json::from_str(&json)?;
        assert_eq!(decoded.id, user.id);
        assert_eq!(decoded.pin_status, Some(PinStatus::Enabled));
        Ok(())
    }

    #[test]
    fn create_user_request_serializes() -> Result<(), Box<dyn std::error::Error>> {
        let req = CreateUserRequest { user_id: "user-abc".to_string() };
        let json = serde_json::to_string(&req)?;
        assert!(json.contains("userId"), "expected camelCase userId in {json}");
        Ok(())
    }
}
