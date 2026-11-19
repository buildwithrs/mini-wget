use std::{
    fs::OpenOptions,
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
    path::PathBuf,
    time::Duration,
};

use reqwest::{
    StatusCode,
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};

use crate::{DLErrors, DLResult};

pub struct Downloader {
    pub url: String,
    pub dest: PathBuf,
    pub http_cli: Client,
}

impl Downloader {
    pub fn new(url: &str, dest: PathBuf) -> DLResult<Self> {
        Ok(Self {
            url: url.into(),
            dest,
            http_cli: ClientBuilder::new()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(5))
                .redirect(Policy::limited(3))
                .build()?,
        })
    }

    pub fn download(&self) -> DLResult<()> {
        let stream = self.http_cli.get(&self.url).send()?;
        if stream.status() != StatusCode::OK {
            return Err(DLErrors::RespError(format!("{}", stream.status())));
        }

        let total_length = match stream.headers().get("Content-Length") {
            Some(total) => match total.to_str() {
                Ok(sl) => Some(sl.parse::<u64>()?),
                Err(_) => None,
            },
            None => None,
        };

        println!("total length of content: {:?}", total_length);

        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.dest)?;

        let mut reader = BufReader::new(stream);
        let mut writer = BufWriter::new(f);
        let mut received_sz = 0u64;

        let mut buf = [0u8; 1014];

        loop {
            let read_sz = match reader.read(&mut buf) {
                Ok(0) => break, // no data.
                Ok(n) => n,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            };

            received_sz += read_sz as u64;

            writer.write_all(&buf[..read_sz])?;
            match total_length {
                Some(total) => {
                    print!("\r received: {}  / total: {}", received_sz, total);
                    io::stdout().flush()?;
                }
                None => {
                    print!("\r received: {}", received_sz);
                    io::stdout().flush()?;
                }
            }
        }

        writer.flush()?;

        println!();
        println!(
            "finish download to: {}, size: {}",
            self.dest.display(),
            received_sz
        );
        Ok(())
    }
}
