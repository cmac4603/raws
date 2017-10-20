extern crate rusoto_core;
extern crate rusoto_ec2;

use std::env;
use rusoto_core::default_tls_client;
use rusoto_core::{ProfileProvider, Region};
use rusoto_ec2::{Ec2, Ec2Client, DescribeInstancesRequest};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let account = args[1].as_ref();
    let provider = ProfileProvider::with_configuration("/Users/cmacrae/.aws/credentials", account);
    let client = Ec2Client::new(default_tls_client().unwrap(), provider, Region::EuWest1);
    let list_ec2_input: DescribeInstancesRequest = Default::default();

    match client.describe_instances(&list_ec2_input) {
        Ok(output) => {
            match output.reservations {
                Some(reservations_list) => {
                    for reservations in reservations_list {
                        // println!("{:?}", reservations.instances.iter());
                        for instance in reservations.instances.iter() {
                            let ec2_details = instance.get(0).unwrap();
                            let ec2_tags = ec2_details.tags.clone().unwrap();
                            for tags in ec2_tags {
                                match tags.key {
                                    Some(tag_key) => {
                                        if tag_key == "Name" {
                                            let status = ec2_details.state.clone().unwrap();
                                            let status_result = status.name.unwrap();
                                            println!("{} has public IP {} and private IP {} - {}",
                                                     tags.value.unwrap_or(String::from("No name")),
                                                     ec2_details
                                                         .public_ip_address
                                                         .clone()
                                                         .unwrap_or(String::from("NONE",),),
                                                     ec2_details
                                                         .private_ip_address
                                                         .clone()
                                                         .unwrap_or(String::from("NONE",),),
                                                     status_result,)
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                None => println!("No instances found!"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
