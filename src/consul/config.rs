pub struct Config {
    pub address: String,
    pub token: String,
    pub certificate: String,
    // If certificate property is empty, the certificate value will be read from the file at certificate_path
    pub certificate_path: String
}

// If certificate param is empty, the certificate value will be read from the file at certificate_path
pub fn new(address: String, token: String, certificate: String, certificate_path: String) -> Config {
    Config {
        address,
        token,
        certificate,
        certificate_path
    }
}