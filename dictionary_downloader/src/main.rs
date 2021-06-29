use sequence_trie::SequenceTrie;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Dictionary Builder")]
struct Opt {
    #[structopt(
        short = "u",
        long,
        help = "URL from which to download the dictionary artifact",
        default_value = "http://zyzzyva.net/packages/324/NASPAZyzzyva324-64.tar.gz",
        parse(try_from_str)
    )]
    input_url: hyper::Uri,

    #[structopt(
        short = "p",
        long,
        help = "Path in archive where to find dictionary",
        default_value = "."
    )]
    input_subpath: String,

    #[structopt(
        short = "t",
        long,
        parse(from_os_str),
        default_value = "~/Downloads",
        help = "Where to store the downloaded dictionary artifact (and its extraction)"
    )]
    input_tmpdir: PathBuf,

    #[structopt(
        short = "o",
        long,
        parse(from_os_str),
        help = "Where to output the serialized trie",
        default_value = "./dictionary.bin"
    )]
    output: PathBuf,
}

type AResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct Cli {
    opts: Opt,
}

#[tokio::main]
async fn main() -> AResult<()> {
    let this = Cli {
        opts: Opt::from_args(),
    };

    let artifact_info = this.get_artifact_info().await?;

    let artifact_path = this.opts.input_tmpdir.join(artifact_info.name);
    if !artifact_path.exists() {
        println!(
            "Downloading dictionary artifact.\n From: {}\n To: {}",
            this.opts.input_url,
            artifact_path.to_str().unwrap()
        );
        // this.download_artifact(artifact_info.size, &artifact_path)
        //     .await?;
    }

    Ok(())
}

#[derive(Debug)]
struct ArtifactInfo {
    size: usize,
    name: String,
}
impl Cli {
    async fn get_artifact_info(&self) -> AResult<ArtifactInfo> {
        let mut res = hyper::Client::new()
            .request(
                hyper::Request::head(&self.opts.input_url)
                    .body(hyper::Body::empty())
                    .unwrap(),
            )
            .await?;

        Ok(ArtifactInfo {
            size: res
                .headers_mut()
                .get_mut(hyper::header::CONTENT_LENGTH)
                .expect("Server didn't specify size")
                .to_str()?
                .parse()?,
            name: if res
                .headers()
                .get(hyper::header::CONTENT_DISPOSITION)
                .is_some()
                && res
                    .headers()
                    .get(hyper::header::CONTENT_DISPOSITION)
                    .unwrap()
                    .to_str()?
                    .contains("filename")
            {
                res.headers_mut()
                    .get_mut(hyper::header::CONTENT_DISPOSITION)
                    .unwrap()
                    .to_str()?
                    .split(";")
                    .find(|c| c.starts_with("filename"))
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .trim_matches('"')
                    .to_owned()
            } else {
                self.opts
                    .input_url
                    .path()
                    .split('/')
                    .last()
                    .expect("Url contained no path?")
                    .to_owned()
            },
        })
    }

    // async fn download_artifact(&self, expected_size: usize, output: &PathBuf) -> AResult<()> {
    //     let mut file = tokio::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .open(output)
    //         .await?;
    //
    // }
}
