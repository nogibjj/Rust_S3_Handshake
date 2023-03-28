use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::fs::File;
use std::io::Read;

fn main() {
    let bucket_name = "your-bucket-name";
    let file_path = "/path/to/your/file.csv";
    let object_key = "file.csv";

    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let s3_client = S3Client::new(Region::UsEast1);
    let request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: object_key.to_owned(),
        body: Some(contents.into()),
        ..Default::default()
    };

    match s3_client.put_object(request).sync() {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => println!("Error uploading file: {:?}", e),
    }
}