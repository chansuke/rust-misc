fn image_handler(req: Request<Body>, files: &Path) -> Box<Future<Item=Response<Body>, Error=Error> + Send> {
    match (req.method(), req.uri().path().to_owned().as_ref()) {
        (&Method::GET, "/") => {
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::GET, path) if path.starts_with("/download") => {
            if let Some(cap) = DOWNLOAD_FILE.captures(path) {
                let filename = cap.name("filename").unwrap().as_str();
                let mut filepath = files.to_path_buf();
                filepath.push(filename);
                let open_file = File::open(filepath);
                let body = open_file.map(|file| {
                    let chunks = FileChunkStream::new(file);
                    Response::new(Body::wrap_stream(chunks))
                });
                Box::new(body)
            } else {
                response_with_code(StatusCode::NOT_FOUND)
            }
        },
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        },
    }
}

fn response_with_code(status_code: StatusCode) -> Box<Future<Item=Response<Body>, Error=Error> + Send> {
    let resp = Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap();
    Box::new(future::ok(resp))
}

fn main() {
    let images = Path::new("./images");
    fs::create_dir(files).ok();
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(move || {
        service_fn(move |req| image_handler(req, &files))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
