

pub struct Common {
    pub nn_hostport: String,
    pub effective_user: String
}

impl Default for Common {
    fn default() -> Self {
        Common {
            nn_hostport: "127.0.0.1:8020".to_owned(),
            effective_user: "cloudera".to_owned()
        }
    }
}

pub struct GetListing {
    pub src: Vec<String>,
    //[-C] [-d] [-h] [-q] [-R] [-t] [-S] [-r] [-u]
    pub c_u: bool,
    pub d: bool,
    pub h: bool,
    pub q: bool,
    pub r_u: bool,
    pub t: bool,
    pub s_u: bool,
    pub r: bool,
    pub u: bool
}

impl Default for GetListing {
    fn default() -> Self {
        GetListing {
            src: vec![], c_u: false, d: false, h: false, q: false, r_u: false,
            t: false, s_u: false, r: false, u: false
        }
    }
}