

pub struct Common {
    pub nn_host: String,
    pub nn_port: u16,
    pub effective_user: String
}

impl Default for Common {
    fn default() -> Self {
        Common {
            nn_host: "127.0.0.1".to_owned(),
            nn_port: 8020,
            effective_user: "cloudera".to_owned()
        }
    }
}

pub struct GetListing {
    pub src: Vec<String>,
    pub need_location: bool
}

pub struct Get {
    pub src: Vec<String>,
    pub tgt_dir: Option<String>
}

