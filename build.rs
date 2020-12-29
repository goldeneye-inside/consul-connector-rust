fn main()->Result<(),Box<dyn std::error::Error>>{
    // Compiling protos using path on build time
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["protos/connector.proto"],
            &["protos"],
        )?;
    Ok(())
}
