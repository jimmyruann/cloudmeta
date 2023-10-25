static TEST_BASE_URL: &str = "http://127.0.0.1:1338/latest/";

#[tokio::test]
async fn test_connectivity() {
    use aws::AWSInstanceMetadata;

    let test_instance: AWSInstanceMetadata = AWSInstanceMetadata::new(Some(TEST_BASE_URL));

    assert!(test_instance.check_connectivity().await == true)
}

#[tokio::test]
async fn test_instance_info() {
    use aws::{AWSInstanceMetadata, AvailabilityZoneInfo, InstanceInfo};

    let test_instance: AWSInstanceMetadata = AWSInstanceMetadata::new(Some(TEST_BASE_URL));

    let expected: InstanceInfo = InstanceInfo {
        instance_id: String::from("i-1234567890abcdef0"),
        instance_type: String::from("m4.xlarge"),
        ami_id: String::from("ami-0a887e401f7654935"),
        local_hostname: String::from("ip-172-16-34-43.ec2.internal"),
        local_ipv4: String::from("172.16.34.43"),
        public_hostname: String::from("ec2-192-0-2-54.compute-1.amazonaws.com"),
        public_ipv4: String::from("192.0.2.54"),
        mac_address: vec![String::from("0e:49:61:0f:c3:11")],
        hostname: String::from("ip-172-16-34-43.ec2.internal"),
        region: String::from("us-east-1"),
        availability_zone: AvailabilityZoneInfo {
            zone_id: String::from("use1-az4"),
            zone_name: String::from("us-east-1a"),
        },
        ramdisk_id: String::from("ari-01bb5768"),
        tags: vec![String::from("Name"), String::from("Test")],
    };

    assert!(test_instance.get_instance_info().await.eq(&expected));
}
