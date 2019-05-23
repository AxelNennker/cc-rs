use std::string::ToString;
use std::path::{PathBuf, Path};
use std::{env, fs};
use std::process::Command;

use ErrorKind;
use Error;

const ANDROID_AARCH64_DEFAULT_API: &str = "21";
const ANDROID_ARM_DEFAULT_API: &str = "14";

// https://developer.android.com/ndk/guides/other_build_systems
fn android_append_host_tag(ndk: &PathBuf) -> Result<PathBuf, Error> {
    // $NDK/toolchains/llvm/prebuilt
    let path = ndk.join("toolchains").join("llvm").join("prebuild");
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(filetype) = entry.file_type() {
                    if filetype.is_dir() {
                        return Ok(entry.path().clone());
                    }
                }
            }
        }
    }
    Err(Error { kind: ErrorKind::IOError, message: "$NDK/toolchains/llvm/prebuild/something not found".to_string() })
}

// https://developer.android.com/ndk/guides/other_build_systems
fn android_get_ndk() -> Result<PathBuf, Error> {
    // $HOME/Android/Sdk/ndk-bundle
    if let Ok(home) = env::var("HOME") {
        let ndk = Path::new(&home)
            .join("Android")
            .join("Sdk")
            .join("ndk-bundle");
        if ndk.exists() {
            return Ok(ndk);
        } else {
            return Err(Error { kind: ErrorKind::IOError, message: "$HOME/Android/Sdk/ndk-bundle does not exist".to_string() });
        }
    }
    Err(Error { kind: ErrorKind::IOError, message: "$NDK/toolchains/llvm/prebuild/something not found".to_string() })
}

fn android_get_base_compiler(clang_compiler: &str, ndk: &PathBuf) -> Result<PathBuf, Error> {
    // Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/
    let path = ndk
        .join("toolchains")
        .join("llvm")
        .join("prebuild");
    let host_tag = android_append_host_tag(&path);
    match host_tag {
        Ok(val) => {
            val.join(clang_compiler).join("bin");
            if val.exists() && val.is_dir() {
                return Ok(val);
            }
            return Err(Error { kind: ErrorKind::IOError, message: "host_tag not found".to_string() });
        }
        Err(_) => return Err(Error { kind: ErrorKind::IOError, message: "host_tag not found".to_string() })
    }
}

pub fn android_find_compiler(target: &str, gnu: &str, clang: &str) -> String {
    let target = target
        .replace("armv7neon", "arm")
        .replace("armv7", "arm")
        .replace("thumbv7neon", "arm")
        .replace("thumbv7", "arm");
    let android_api: String = match env::var("ANDROID_API") {
        Ok(val) => val,
        Err(_) => {
            if target.contains("aarch64") {
                ANDROID_AARCH64_DEFAULT_API.to_string()
            } else if target.contains("arm") {
                ANDROID_ARM_DEFAULT_API.to_string()
            } else {
                "".to_string()
            }
        }
    };
    let gnu_compiler = format!("{}-{}", target, gnu);
    let clang_compiler = format!("{}{}-{}", target, android_api, clang);
    println!("clang_compiler = {:?}", &clang_compiler);
    if Command::new(&clang_compiler).spawn().is_ok() {
        clang_compiler
    } else {
        match android_get_ndk() {
            Ok(val) => {
                let clang_with_path = android_get_base_compiler(&clang_compiler, &val);
                match clang_with_path {
                    Ok(val) => {
                        println!("clang_with_path = {:?}", val.to_str());
                        if Command::new(&val).spawn().is_ok() {
                            String::from(val.to_str().unwrap())
                        } else {
                            gnu_compiler
                        }
                    }
                    Err(_) => gnu_compiler
                }
            }
            Err(_) => gnu_compiler
        }
    }
}

#[cfg(target_os = "android")]
#[test]
fn test_android_append_host_tag() {
    if let Ok(ndk) = env::var("NDK") {
        let ndk= Path::new(&ndk);
        let host_tag = android_append_host_tag(&ndk.to_path_buf());
        assert!(host_tag.unwrap().is_dir());
    } else {
        let ndk = android_get_ndk();
        match ndk {
            Ok(val) => {
                let host_tag = android_append_host_tag(&val);
                assert!(host_tag.unwrap().is_dir());
            },
            Err(_) => {}
        }
//        let target = env::var("TARGET").unwrap();
    }
}
