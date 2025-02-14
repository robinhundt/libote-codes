use std::path::Path;
use std::{env, fs};

/// This build script builds the required parts of libOTe and cryptotools necessary
/// for the Silver/EaCode/ExConvCode codes for silent OT.
fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libOTe");
    println!("cargo:rerun-if-changed=thirdparty");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    fs::create_dir_all(out_dir.join("libOTe/libOTe")).unwrap();
    fs::create_dir_all(out_dir.join("cryptoTools/cryptoTools/Common")).unwrap();
    fs::copy(
        "src/libOTe_config.h",
        out_dir.join("libOTe/libOTe/config.h"),
    )
    .unwrap();
    fs::copy(
        "src/cryptoTools_config.h",
        out_dir.join("cryptoTools/cryptoTools/Common/config.h"),
    )
    .unwrap();

    let (sse2_enabled, aes_enabled, avx_enabled, pclmul_enabled) =
        if &env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "x86_64" {
            (false, false, false, false)
        } else {
            let env = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
            let target_features: Vec<_> = env.split(',').collect();
            let sse2 = target_features.contains(&"sse2");
            let aes = target_features.contains(&"aes");
            let avx2 = target_features.contains(&"avx2");
            let pclmul = target_features.contains(&"pclmulqdq");
            (sse2, aes, avx2, pclmul)
        };

    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .file("libOTe/cryptoTools/cryptoTools/Common/Timer.cpp")
        .file("libOTe/cryptoTools/cryptoTools/Common/Log.cpp")
        .file("libOTe/cryptoTools/cryptoTools/Common/block.cpp")
        .file("libOTe/cryptoTools/cryptoTools/Crypto/PRNG.cpp")
        .file("libOTe/cryptoTools/cryptoTools/Crypto/AES.cpp")
        .includes(&[
            Path::new("src"),
            Path::new("libOTe"),
            Path::new("libOTe/cryptoTools"),
            Path::new("thirdparty/libdivide"),
            out_dir.join("libOTe").as_path(),
            out_dir.join("cryptoTools").as_path(),
        ])
        .warnings(false)
        .flag("-std=c++20")
        .flag_if_supported("-march=native");

    if env::var("OPT_LEVEL").expect("OPT_LEVEL not set") != "0" {
        build.define("NDEBUG", None);
    }

    if sse2_enabled {
        build
            .define("ENABLE_SSE", None)
            .define("OC_ENABLE_SSE2", None);

        if !pclmul_enabled {
            panic!("if targe_feature = sse2 is enabled, pclmulqdq must be enabled as well")
        }
        build.define("OC_ENABLE_PCLMUL", None);
        build.flag("-mpclmul");
    }

    if avx_enabled {
        build.define("ENABLE_AVX", None);
    }

    if aes_enabled {
        build.define("OC_ENABLE_AESNI", None);
    } else {
        build.define("OC_ENABLE_PORTABLE_AES", None);
    }

    build.compile("silent_encoder_bridge");
}
