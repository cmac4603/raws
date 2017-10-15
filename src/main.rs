extern crate rusoto_core;
extern crate rusoto_ec2;

// use std::env;
use rusoto_core::default_tls_client;
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_ec2::{Ec2, Ec2Client, DescribeInstancesRequest};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = Ec2Client::new(default_tls_client().unwrap(), provider, Region::EuWest1);
    let list_ec2_input: DescribeInstancesRequest = Default::default();

    match client.describe_instances(&list_ec2_input) {
        Ok(output) => {
            match output.reservations {
                Some(reservations_list) => {
                    for reservations in reservations_list {
                        println!("{:?}", reservations.instances.unwrap());
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
