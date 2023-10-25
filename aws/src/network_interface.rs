use surf::Client;

#[derive(Clone, PartialEq, Debug)]
pub struct SecurityGroupInfo {
    pub id: String,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SubnetInfo {
    pub id: String,
    pub ipv4_cidr_block: String,
    pub ipv6_cidr_block: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VpcInfo {
    pub id: String,
    pub ipv4_cidr_block: String,
    pub ipv4_cidr_blocks: String,
    pub ipv6_cidr_blocks: String,
}

pub struct NetworkInterfaceInfo<'a> {
    mac_address: String,
    client: &'a Client,
}

impl<'a> NetworkInterfaceInfo<'a> {
    pub fn new(client: &'a Client, mac_address: String) -> Self {
        Self {
            client,
            mac_address,
        }
    }

    pub async fn get_device_number(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/device-number",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_interface_id(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/interface-id",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_local_hostname(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/local-hostname",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_local_ipv4s(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/local-ipv4s",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_mac(&self) -> String {
        self.client
            .get(format!("network/interfaces/macs/{}/mac", self.mac_address))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_network_card_index(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/network-card-index",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_owner_id(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/owner-id",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_public_hostname(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/public-hostname",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_public_ipv4s(&self) -> String {
        self.client
            .get(format!(
                "network/interfaces/macs/{}/public-ipv4s",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_security_group_info(&self) -> SecurityGroupInfo {
        let id = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/security-group-ids",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let name = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/security-group",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        return SecurityGroupInfo { id, name };
    }

    pub async fn get_subnet_info(&self) -> SubnetInfo {
        let id = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/subnet-id",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let ipv4_cidr_block = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/subnet-ipv4-cidr-block",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let ipv6_cidr_block = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/subnet-ipv6-cidr-blocks",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        return SubnetInfo {
            id,
            ipv4_cidr_block,
            ipv6_cidr_block,
        };
    }

    pub async fn get_vpc_info(&self) -> VpcInfo {
        let id = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/vpc-id",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let ipv4_cidr_block = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/vpc-ipv4-cidr-block",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let ipv4_cidr_blocks = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/vpc-ipv4-cidr-blocks",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        let ipv6_cidr_blocks = self
            .client
            .get(format!(
                "network/interfaces/macs/{}/vpc-ipv6-cidr-blocks",
                self.mac_address
            ))
            .recv_string()
            .await
            .unwrap();

        return VpcInfo {
            id,
            ipv4_cidr_block,
            ipv4_cidr_blocks,
            ipv6_cidr_blocks,
        };
    }
}
