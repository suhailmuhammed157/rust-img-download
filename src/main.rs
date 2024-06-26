use error_chain::error_chain;
use std::env;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let project_dir = env::current_dir()?;
    let tmp_dir = Builder::new().prefix("example").tempdir_in(project_dir)?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let fname = response.url().path_segments().unwrap();

    println!("{:#?}", fname);

    //store in a destination as with a filename
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
