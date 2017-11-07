extern crate rusoto_core;
extern crate rusoto_ec2;

use std::env;
use rusoto_core::default_tls_client;
use rusoto_core::{ProfileProvider, Region};
use rusoto_ec2::{Ec2, Ec2Client, DescribeInstancesRequest};

fn region_selector(acc: &str) -> Region {
    match acc {
        "eu-west-1" => {
            let region: Region = Region::EuWest1;
            region
        },
        "eu-west-2" => {
            let region: Region = Region::EuWest2;
            region
        },
        "us-east-1" => {
            let region: Region = Region::UsEast1;
            region
        },
        "us-east-2" => {
            let region: Region = Region::UsEast2;
            region
        },
        "us-west-1" => {
            let region: Region = Region::UsWest1;
            region
        },
        "us-west-2" => {
            let region: Region = Region::UsWest2;
            region
        },
        _ => {
            let region = Region::UsEast1;
            region
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let account: &str = args[1].as_ref();
    let region_name: &str = args[2].as_ref();
    let region: Region = region_selector(region_name);
    let aws_creds_dir: String = env::home_dir().unwrap().to_str().unwrap().to_owned() + "/.aws/credentials";
    let provider: ProfileProvider = ProfileProvider::with_configuration(aws_creds_dir, account);
    let client = Ec2Client::new(default_tls_client().unwrap(), provider, region);
    let list_ec2_input: DescribeInstancesRequest = Default::default();

    match client.describe_instances(&list_ec2_input) {
        Ok(output) => match output.reservations {
            Some(reservations_list) => for reservations in reservations_list {
                // println!("{:?}", reservations.instances.iter());
                for instance in reservations.instances.iter() {
                    let ec2_details: &rusoto_ec2::Instance = instance.get(0).unwrap();
                    let ec2_tags: Vec<rusoto_ec2::Tag> = ec2_details.tags.clone()
                        .unwrap_or(vec![rusoto_ec2::Tag { key: Some(String::from("Name")),
                                                          value: Some(String::from("NO_NAME"))}]);
                    for tags in ec2_tags {
                        match tags.key {
                            Some(tag_key) => if tag_key == "Name" {
                                let instance_name: String =
                                    tags.value.unwrap_or(String::from("NO_NAME"));
                                let public_ip_address: String =
                                    ec2_details
                                        .public_ip_address
                                        .clone()
                                        .unwrap_or(String::from("NONE"));
                                let private_ip_address: String =
                                    ec2_details
                                        .private_ip_address
                                        .clone()
                                        .unwrap_or(String::from("NONE"));
                                let status: String =
                                    ec2_details.state.clone().unwrap()
                                        .name.unwrap_or(String::from("status_unknown"));
                                println!("{} has public IP {} and private IP {} - {}",
                                         instance_name,
                                         public_ip_address,
                                         private_ip_address,
                                         status,)
                            }
                            _ => (),
                        }
                    }
                }
            }
            None => println!("No instances found!"),
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
