use rusoto_core::{Region, RusotoError};
use rusoto_credential::{AwsCredentials, StaticProvider};
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::fs::File;
use std::io::Read;
use tokio::runtime::Runtime;

fn main() {
    let bucket_name = "rust-s3-handshake";
    let file_path = "spam_text_data.csv";
    let object_key = "file.csv";

    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let access_key_id = std::env::var("AWS_ACCESS_KEY").unwrap();
    let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY").unwrap();
    let credentials = AwsCredentials::new(
        access_key_id,
        secret_access_key,
        None,
        None,
    );
    let s3_client = S3Client::new_with(
        rusoto_core::request::HttpClient::new().unwrap(),
        StaticProvider::from(credentials),
        Region::UsEast1,
    );
    let request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: object_key.to_owned(),
        body: Some(contents.into()),
        ..Default::default()
    };

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match s3_client.put_object(request).await {
            Ok(_) => println!("File uploaded successfully"),
            Err(e) => println!("Error uploading file: {:?}", e),
        }
    });
}