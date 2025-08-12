fn main() {
    // let ac = autocfg::new();
    // ac.set_no_std(true);
    // ac.
    println!(
        "cargo::rustc-env=HAL_ENV_ARCH={}",
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    );

    let platform = std::env::var("POLYHAL_PLATFORM").unwrap_or(String::from("default"));
    let cpu_family = std::env::var("POLYHAL_CPU_FAMILY").unwrap_or(String::from("default"));
    println!("cargo::rustc-cfg=platform=\"{}\"", platform);
    println!("cargo::rustc-cfg=cpu_family=\"{}\"", cpu_family);
    println!("cargo::rustc-check-cfg=cfg(cpu_family, values(\"c906\"))");

    // set_var(
    //     "HAL_ENV_ARCH",
    //     std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
    // );
}
