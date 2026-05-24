pub struct ErrorCode {
    pub code: &'static str,
    pub message: &'static str,
}

pub const USER_NOT_FOUND: ErrorCode = ErrorCode {
    code: "USER_NOT_FOUND",
    message: "user not found",
};

pub const EMAIL_ALREADY_EXISTS: ErrorCode = ErrorCode {
    code: "EMAIL_ALREADY_EXISTS",
    message: "email already exists",
};

pub const INCORRECT_PASSWORD: ErrorCode = ErrorCode {
    code: "INCORRECT_PASSWORD",
    message: "incorrect password",
};

pub const NOT_PASSWORD_AUTH_USER: ErrorCode = ErrorCode {
    code: "NOT_PASSWORD_AUTH_USER",
    message: "not a password auth user",
};

pub const INVALID_EMAIL: ErrorCode = ErrorCode {
    code: "INVALID_EMAIL",
    message: "invalid email",
};

pub const INVALID_USER_NAME: ErrorCode = ErrorCode {
    code: "INVALID_USER_NAME",
    message: "INVALID_USER_NAME",
};

pub const UNAUTHORIZED: ErrorCode = ErrorCode {
    code: "UNAUTHORIZED",
    message: "unauthorized",
};

pub const INTERNAL_SERVER_ERROR: ErrorCode = ErrorCode {
    code: "INTERNAL_SERVER_ERROR",
    message: "internal server error",
};

pub const INVALID_REQUEST: ErrorCode = ErrorCode {
    code: "INVALID_REQUEST",
    message: "invalid request",
};
