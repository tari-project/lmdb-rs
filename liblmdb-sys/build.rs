use std::env;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    let mut config = cc::Build::new();
    config.file("mdb/libraries/liblmdb/mdb.c")
          .file("mdb/libraries/liblmdb/midl.c");
    config.opt_level(2);
    config.flag_if_supported("-Wno-unused-parameter");
    config.flag_if_supported("-Wno-implicit-fallthrough");

    if target.contains("dragonfly") {
        config.flag("-DMDB_DSYNC=O_SYNC");
        config.flag("-DMDB_FDATASYNC=fsync");
    }

    if let Ok(target_arch) = env::var("LMDB_TARGET") {
        //config.flag_if_supported(&format!("-march={}", target_arch));
        config.flag(&format!("--target={}", target_arch));
    }

    config.compile("liblmdb.a");
}
