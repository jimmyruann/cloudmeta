mod network_interface;

use network_interface::NetworkInterfaceInfo;
use std::time::Duration;
use surf::{Client, Config, Url};

#[derive(Clone, PartialEq, Debug)]
pub struct InstanceInfo {
    pub ami_id: String,
    pub instance_id: String,
    pub instance_type: String,
    pub hostname: String,
    pub local_hostname: String,
    pub local_ipv4: String,
    pub public_hostname: String,
    pub public_ipv4: String,
    pub mac_address: Vec<String>,
    pub region: String,
    pub availability_zone: AvailabilityZoneInfo,
    pub ramdisk_id: String,
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AvailabilityZoneInfo {
    pub zone_name: String,
    pub zone_id: String,
}

pub struct AWSInstanceMetadata {
    client: Client,
}

impl AWSInstanceMetadata {
    pub fn new(base_url: Option<&str>) -> AWSInstanceMetadata {
        let b_url = base_url.unwrap_or("http://169.254.169.254/2022-09-24/");

        let client: Client = Config::new()
            .set_base_url(Url::parse(b_url).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();

        return Self { client };
    }

    pub async fn check_connectivity(&self) -> bool {
        let res = self.client.get("").recv_string().await;
        match res {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn get_instance_id(&self) -> String {
        self.client
            .get("meta-data/instance-id")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_instance_type(&self) -> String {
        self.client
            .get("meta-data/instance-type")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_machine_image_id(&self) -> String {
        self.client
            .get("meta-data/ami-id")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_local_hostname(&self) -> String {
        self.client
            .get("meta-data/local-hostname")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_local_ipv4(&self) -> String {
        self.client
            .get("meta-data/local-ipv4")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_public_hostname(&self) -> String {
        self.client
            .get("meta-data/public-hostname")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_public_ipv4(&self) -> String {
        self.client
            .get("meta-data/public-ipv4")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_mac(&self) -> Vec<String> {
        self.client
            .get("meta-data/mac")
            .recv_string()
            .await
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    pub async fn get_hostname(&self) -> String {
        self.client
            .get("meta-data/hostname")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_kernel_id(&self) -> String {
        self.client
            .get("meta-data/kernel-id")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_security_groups(&self) -> String {
        self.client
            .get("meta-data/security-groups")
            .recv_string()
            .await
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    pub async fn get_ramdisk_id(&self) -> String {
        self.client
            .get("meta-data/ramdisk-id")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_region(&self) -> String {
        self.client
            .get("meta-data/placement/region")
            .recv_string()
            .await
            .unwrap()
    }

    pub async fn get_availability_zone(&self) -> AvailabilityZoneInfo {
        let zone_name = self
            .client
            .get("meta-data/placement/availability-zone")
            .recv_string()
            .await
            .unwrap();

        let zone_id = self
            .client
            .get("meta-data/placement/availability-zone-id")
            .recv_string()
            .await
            .unwrap();

        return AvailabilityZoneInfo { zone_id, zone_name };
    }

    pub async fn get_instance_tags(&self) -> Vec<String> {
        self.client
            .get("meta-data/tags/instance")
            .recv_string()
            .await
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    pub async fn get_network_interfaces(&self) -> Vec<NetworkInterfaceInfo> {
        let macs = self.get_mac().await;

        let mut results: Vec<NetworkInterfaceInfo> = vec![];

        for mac in macs {
            results.push(NetworkInterfaceInfo::new(&self.client, mac))
        }

        return results;
    }

    pub async fn get_instance_info(&self) -> InstanceInfo {
        let instance_id = self.get_instance_id().await;
        let instance_type = self.get_instance_type().await;
        let machine_image_id = self.get_machine_image_id().await;
        let local_hostname = self.get_local_hostname().await;
        let local_ipv4 = self.get_local_ipv4().await;
        let public_hostname = self.get_public_hostname().await;
        let public_ipv4 = self.get_public_ipv4().await;
        let mac_address = self.get_mac().await;
        let hostname = self.get_hostname().await;
        let region = self.get_region().await;
        let availability_zone = self.get_availability_zone().await;
        let ramdisk_id = self.get_ramdisk_id().await;
        let tags = self.get_instance_tags().await;

        return InstanceInfo {
            instance_id,
            instance_type,
            ami_id: machine_image_id,
            local_hostname,
            local_ipv4,
            public_hostname,
            public_ipv4,
            mac_address,
            hostname,
            region,
            availability_zone,
            ramdisk_id,
            tags,
        };
    }
}
