use std::fs;
use std::error::Error;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use rpc::consul_connector_client::ConsulConnectorClient;
use rpc::{
    ReqRegisterService,
    ReqGetServiceAddress, RespGetServiceAddress
};

use super::config::Config;

mod rpc {
    tonic::include_proto!("consul"); // The string specified here must match the proto package name
}

pub struct Connector {
    cert: String,
    config: Config,
}

pub fn new(config: Config) -> Connector {
    let mut cert = config.certificate.clone();
    if cert.len() == 0 {
        cert = match fs::read_to_string(&config.certificate_path) {
            Ok(val) => val,
            Err(e) => panic!("Error: {}", e)
        }
    }
    Connector {
        cert,
        config,
    }
}

impl Connector {
    async fn connect_channel(&self) -> Result<Channel, Box<dyn Error>> {
        // Build channel
        let channel = match Channel::from_shared(self.config.address.clone()) {
            Ok(val) => val,
            Err(e) => return Err(Box::new(e))
        };
        let tls = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(self.cert.clone()))
            .domain_name(self.config.domain.clone());
        let endpoint = match channel.tls_config(tls) {
            Ok(val) => val,
            Err(e) => return Err(Box::new(e))
        };

        // Connect to channel
        match endpoint.connect().await {
            Ok(val) => Ok(val),
            Err(e) => Err(Box::new(e))
        }
    }

    #[allow(dead_code)]
    pub async fn register_service(&self, host_name: String, service_name: String, service_ip: String, service_port: u32, health_check_url: String) -> Result<(), Box<dyn Error>> {
        let mut client = match self.connect_channel().await {
            Ok(channel) => ConsulConnectorClient::new(channel),
            Err(e) => return Err(e)
        };
        let request = tonic::Request::new(ReqRegisterService {
            consul_token: self.config.token.clone(),
            service_name,
            service_id: format!("{}-{}", host_name, service_port),
            service_ip,
            service_port,
            health_check_url
        });
        match client.register_service(request).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    #[allow(dead_code)]
    pub async fn get_service_address(&self, service_name: String) -> Result<String, Box<dyn Error>> {
        let mut client = match self.connect_channel().await {
            Ok(channel) => ConsulConnectorClient::new(channel),
            Err(e) => return Err(e)
        };
        let request = tonic::Request::new(ReqGetServiceAddress {
            consul_token: self.config.token.clone(),
            service_name,
        });
        let response = match client.get_service_address(request).await {
            Ok(val) => val,
            Err(e) => return Err(Box::new(e))
        };
        let resp: RespGetServiceAddress = response.into_inner();
        return Ok(resp.address);
    }
}