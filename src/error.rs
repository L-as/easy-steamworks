use derive_more::Display;
use erroneous::Error as EError;

/// This value is returned from many Steam API functions, and is akin to `Result<(), steam::Error>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawResult(pub u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, Display, EError, PartialEq, Eq)]
pub enum Error {
	#[display(fmt = "Failed")]
	Fail,
	#[display(fmt = "No connection")]
	NoConnection,
	#[display(fmt = "Invalid password")]
	InvalidPassword,
	#[display(fmt = "You are already logged in elsewhere")]
	LoggedInElsewhere,
	#[display(fmt = "Protocol version is invalid")]
	InvalidProtocolVer,
	#[display(fmt = "Parameter is invalid")]
	InvalidParam,
	#[display(fmt = "File was not found")]
	FileNotFound,
	#[display(fmt = "Steam is busy")]
	Busy,
	#[display(fmt = "State is invalid")]
	InvalidState,
	#[display(fmt = "Name is invalid")]
	InvalidName,
	#[display(fmt = "E-mail is invalid")]
	InvalidEmail,
	#[display(fmt = "Duplicate name")]
	DuplicateName,
	#[display(fmt = "Access denied")]
	AccessDenied,
	#[display(fmt = "Timed out")]
	Timeout,
	#[display(fmt = "You are banned")]
	Banned,
	#[display(fmt = "Account was not found")]
	AccountNotFound,
	#[display(fmt = "Steam ID is invalid")]
	InvalidSteamID,
	#[display(fmt = "Service is unavailable")]
	ServiceUnavailable,
	#[display(fmt = "You are not logged on")]
	NotLoggedOn,
	Pending,
	#[display(fmt = "Encryption failed")]
	EncryptionFailure,
	#[display(fmt = "Insufficient privileges")]
	InsufficientPrivilege,
	#[display(fmt = "Limit exceeded")]
	LimitExceeded,
	#[display(fmt = "Revoked")]
	Revoked,
	#[display(fmt = "Expired")]
	Expired,
	#[display(fmt = "Already redeemed")]
	AlreadyRedeemed,
	#[display(fmt = "Duplicate request")]
	DuplicateRequest,
	#[display(fmt = "Already owned")]
	AlreadyOwned,
	#[display(fmt = "IP was not found")]
	IPNotFound,
	#[display(fmt = "Persist failed")]
	PersistFailed,
	#[display(fmt = "Could not lock")]
	LockingFailed,
	#[display(fmt = "Log-in session was replaced")]
	LogonSessionReplaced,
	#[display(fmt = "Connection failed")]
	ConnectFailed,
	#[display(fmt = "Handshake failed")]
	HandshakeFailed,
	#[display(fmt = "IO error occurred")]
	IOFailure,
	#[display(fmt = "Remote disconnected")]
	RemoteDisconnect,
	#[display(fmt = "Shopping cart was not found")]
	ShoppingCartNotFound,
	Blocked,
	Ignored,
	#[display(fmt = "No match")]
	NoMatch,
	#[display(fmt = "Account disabled")]
	AccountDisabled,
	#[display(fmt = "Service is read-only")]
	ServiceReadOnly,
	#[display(fmt = "Account is not featured")]
	AccountNotFeatured,
	AdministratorOK,
	ContentVersion,
	TryAnotherCM,
	PasswordRequiredToKickSession,
	Suspended,
	Cancelled,
	#[display(fmt = "Data is corrupted")]
	DataCorruption,
	#[display(fmt = "Disk is full")]
	DiskFull,
	#[display(fmt = "Remote call failed")]
	RemoteCallFailed,
	#[display(fmt = "Password is not set")]
	PasswordUnset,
	#[display(fmt = "External account is unlinked")]
	ExternalAccountUnlinked,
	#[display(fmt = "PSN ticket is invalid")]
	PSNTicketInvalid,
	#[display(fmt = "External account is already linked")]
	ExternalAccountAlreadyLinked,
	#[display(fmt = "Remote file conflicted")]
	RemoteFileConflict,
	#[display(fmt = "Password is illegal")]
	IllegalPassword,
	#[display(fmt = "Same as previous value")]
	SameAsPreviousValue,
	#[display(fmt = "Account log-on was denied")]
	AccountLogonDenied,
	#[display(fmt = "You can not use your old password")]
	CannotUseOldPassword,
	#[display(fmt = "Log-in authentication code is invalid")]
	InvalidLoginAuthCode,
	#[display(fmt = "Account log-in was denied due to no e-mail")]
	AccountLogonDeniedNoMail,
	#[display(fmt = "Hardware is not capable of IPT")]
	HardwareNotCapableOfIPT,
	#[display(fmt = "IPT could not be initialized")]
	IPTInitError,
	#[display(fmt = "Parental control restricts this")]
	ParentalControlRestricted,
	#[display(fmt = "Could not query Facebook")]
	FacebookQueryError,
	#[display(fmt = "Log-in authentication code has expired")]
	ExpiredLoginAuthCode,
	#[display(fmt = "IP Log-in restriction failed")]
	IPLoginRestrictionFailed,
	#[display(fmt = "Account is locked down")]
	AccountLockedDown,
	#[display(fmt = "Account log-in was denied due to no verified e-mail")]
	AccountLogonDeniedVerifiedEmailRequired,
	#[display(fmt = "No URL matches")]
	NoMatchingURL,
	#[display(fmt = "Bad response")]
	BadResponse,
	#[display(fmt = "Password must be re-entered")]
	RequirePasswordReEntry,
	#[display(fmt = "Value is out of range")]
	ValueOutOfRange,
	#[display(fmt = "Unexpected error")]
	UnexpectedError,
	Disabled,
	#[display(fmt = "Invalid CEG Submission")]
	InvalidCEGSubmission,
	#[display(fmt = "Device is restricted")]
	RestrictedDevice,
	#[display(fmt = "Region-locked")]
	RegionLocked,
	#[display(fmt = "You have exceeded your rate limit")]
	RateLimitExceeded,
	#[display(fmt = "Account log-in was denied due to no 2FA")]
	AccountLoginDeniedNeedTwoFactor,
	#[display(fmt = "Item has been deleted")]
	ItemDeleted,
	#[display(fmt = "Account log-in was denied due to throttling")]
	AccountLoginDeniedThrottle,
	#[display(fmt = "2FA code mismatched")]
	TwoFactorCodeMismatch,
	#[display(fmt = "2FA activation code mismatched")]
	TwoFactorActivationCodeMismatch,
	#[display(fmt = "Account is associated to multiple partners")]
	AccountAssociatedToMultiplePartners,
	#[display(fmt = "Not modified")]
	NotModified,
	#[display(fmt = "No mobile device")]
	NoMobileDevice,
	#[display(fmt = "Time is not synchronized")]
	TimeNotSynced,
	#[display(fmt = "SMS code failed")]
	SmsCodeFailed,
	#[display(fmt = "Account limit has been exceeded")]
	AccountLimitExceeded,
	#[display(fmt = "Account activity limit has been exceeded")]
	AccountActivityLimitExceeded,
	#[display(fmt = "Phone activity limit has been exceeded")]
	PhoneActivityLimitExceeded,
	RefundToWallet,
	#[display(fmt = "Could not send e-mail")]
	EmailSendFailure,
	NotSettled,
	NeedCaptcha,
	#[display(fmt = "GSLT denied")]
	GSLTDenied,
	#[display(fmt = "GS owner was denied")]
	GSOwnerDenied,
	#[display(fmt = "Item type is invalid")]
	InvalidItemType,
	#[display(fmt = "You are IP banned")]
	IPBanned,
	#[display(fmt = "GSLT is expired")]
	GSLTExpired,
	#[display(fmt = "Funds are insufficient")]
	InsufficientFunds,
	#[display(fmt = "Too many pending")]
	TooManyPending,
	#[display(fmt = "No site licenses found")]
	NoSiteLicensesFound,
	#[display(fmt = "WG network send exceeded")]
	WGNetworkSendExceeded,
}

impl From<RawResult> for Result<(), Error> {
	fn from(r: RawResult) -> Self {
		match r.0 {
			1 => Ok(()),
			2 => Err(Error::Fail),
			3 => Err(Error::NoConnection),
			5 => Err(Error::InvalidPassword),
			6 => Err(Error::LoggedInElsewhere),
			7 => Err(Error::InvalidProtocolVer),
			8 => Err(Error::InvalidParam),
			9 => Err(Error::FileNotFound),
			10 => Err(Error::Busy),
			11 => Err(Error::InvalidState),
			12 => Err(Error::InvalidName),
			13 => Err(Error::InvalidEmail),
			14 => Err(Error::DuplicateName),
			15 => Err(Error::AccessDenied),
			16 => Err(Error::Timeout),
			17 => Err(Error::Banned),
			18 => Err(Error::AccountNotFound),
			19 => Err(Error::InvalidSteamID),
			20 => Err(Error::ServiceUnavailable),
			21 => Err(Error::NotLoggedOn),
			22 => Err(Error::Pending),
			23 => Err(Error::EncryptionFailure),
			24 => Err(Error::InsufficientPrivilege),
			25 => Err(Error::LimitExceeded),
			26 => Err(Error::Revoked),
			27 => Err(Error::Expired),
			28 => Err(Error::AlreadyRedeemed),
			29 => Err(Error::DuplicateRequest),
			30 => Err(Error::AlreadyOwned),
			31 => Err(Error::IPNotFound),
			32 => Err(Error::PersistFailed),
			33 => Err(Error::LockingFailed),
			34 => Err(Error::LogonSessionReplaced),
			35 => Err(Error::ConnectFailed),
			36 => Err(Error::HandshakeFailed),
			37 => Err(Error::IOFailure),
			38 => Err(Error::RemoteDisconnect),
			39 => Err(Error::ShoppingCartNotFound),
			40 => Err(Error::Blocked),
			41 => Err(Error::Ignored),
			42 => Err(Error::NoMatch),
			43 => Err(Error::AccountDisabled),
			44 => Err(Error::ServiceReadOnly),
			45 => Err(Error::AccountNotFeatured),
			46 => Err(Error::AdministratorOK),
			47 => Err(Error::ContentVersion),
			48 => Err(Error::TryAnotherCM),
			49 => Err(Error::PasswordRequiredToKickSession),
			50 => Err(Error::LoggedInElsewhere),
			51 => Err(Error::Suspended),
			52 => Err(Error::Cancelled),
			53 => Err(Error::DataCorruption),
			54 => Err(Error::DiskFull),
			55 => Err(Error::RemoteCallFailed),
			56 => Err(Error::PasswordUnset),
			57 => Err(Error::ExternalAccountUnlinked),
			58 => Err(Error::PSNTicketInvalid),
			59 => Err(Error::ExternalAccountAlreadyLinked),
			60 => Err(Error::RemoteFileConflict),
			61 => Err(Error::IllegalPassword),
			62 => Err(Error::SameAsPreviousValue),
			63 => Err(Error::AccountLogonDenied),
			64 => Err(Error::CannotUseOldPassword),
			65 => Err(Error::InvalidLoginAuthCode),
			66 => Err(Error::AccountLogonDeniedNoMail),
			67 => Err(Error::HardwareNotCapableOfIPT),
			68 => Err(Error::IPTInitError),
			69 => Err(Error::ParentalControlRestricted),
			70 => Err(Error::FacebookQueryError),
			71 => Err(Error::ExpiredLoginAuthCode),
			72 => Err(Error::IPLoginRestrictionFailed),
			73 => Err(Error::AccountLockedDown),
			74 => Err(Error::AccountLogonDeniedVerifiedEmailRequired),
			75 => Err(Error::NoMatchingURL),
			76 => Err(Error::BadResponse),
			77 => Err(Error::RequirePasswordReEntry),
			78 => Err(Error::ValueOutOfRange),
			79 => Err(Error::UnexpectedError),
			80 => Err(Error::Disabled),
			81 => Err(Error::InvalidCEGSubmission),
			82 => Err(Error::RestrictedDevice),
			83 => Err(Error::RegionLocked),
			84 => Err(Error::RateLimitExceeded),
			85 => Err(Error::AccountLoginDeniedNeedTwoFactor),
			86 => Err(Error::ItemDeleted),
			87 => Err(Error::AccountLoginDeniedThrottle),
			88 => Err(Error::TwoFactorCodeMismatch),
			89 => Err(Error::TwoFactorActivationCodeMismatch),
			90 => Err(Error::AccountAssociatedToMultiplePartners),
			91 => Err(Error::NotModified),
			92 => Err(Error::NoMobileDevice),
			93 => Err(Error::TimeNotSynced),
			94 => Err(Error::SmsCodeFailed),
			95 => Err(Error::AccountLimitExceeded),
			96 => Err(Error::AccountActivityLimitExceeded),
			97 => Err(Error::PhoneActivityLimitExceeded),
			98 => Err(Error::RefundToWallet),
			99 => Err(Error::EmailSendFailure),
			100 => Err(Error::NotSettled),
			101 => Err(Error::NeedCaptcha),
			102 => Err(Error::GSLTDenied),
			103 => Err(Error::GSOwnerDenied),
			104 => Err(Error::InvalidItemType),
			105 => Err(Error::IPBanned),
			106 => Err(Error::GSLTExpired),
			107 => Err(Error::InsufficientFunds),
			108 => Err(Error::TooManyPending),
			109 => Err(Error::NoSiteLicensesFound),
			110 => Err(Error::WGNetworkSendExceeded),
			_ => unreachable!(),
		}
	}
}
