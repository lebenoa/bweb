use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    App, HttpServer,
};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The directory to serve
    dir: String,

    /// Address to bind to
    #[clap(short, long, default_value = "127.0.0.1:8998")]
    address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.dir.is_empty() {
        println!("No directory provided");
        return Ok(());
    }

    std::env::set_current_dir(&args.dir)?;

    println!("Serving dir `{}` at `{}`", args.dir, args.address);

    HttpServer::new(move || {
        App::new().service(
            Files::new("/", "./")
                .index_file("index.html")
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let filename = format!("./{}.html", req.path());
                    let file = NamedFile::open_async(filename).await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                })),
        )
    })
    .bind(args.address)?
    .run()
    .await
}
