mod fm_data;
use fm_data::config::Configuration;

/*
extern crate rusoto_core;
extern crate rusoto_s3;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
*/
use std::time::Instant;


fn do_update_s3(opts: &Configuration) {
    /*
    if opts.ingame_date == "NA" {
        println!("--ingame_date not set, skipping S3 upload");
    } else {
        let ingame_date = &opts.ingame_date;
        println!("Setting date to {}", ingame_date);

        let buf: Vec<u8> = "hello world data".into();

        let s3_client = S3Client::new(Region::EuCentral1);
        s3_client
            .put_object(PutObjectRequest {
                bucket: String::from("bjoernd-fm-data"),
                key: String::from("data"),
                body: Some(buf.into()),
                ..Default::default()
            })
            .sync()
            .unwrap();
    }
    */
}

fn main() {
    let start_time = Instant::now();
    let config = fm_data::config::read_configuration().unwrap();
    do_update_s3(&config);
    println!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );
}
