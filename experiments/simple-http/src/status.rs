pub struct Status {
    pub code: u16,
    pub reason: &'static str,
}

pub const OK: Status = Status {
    code: 200,
    reason: "Ok",
};

pub const BAD_REQUEST: Status = Status {
    code: 400,
    reason: "Bad Request",
};

pub const NOT_FOUND: Status = Status {
    code: 404,
    reason: "Not Found",
};

