use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "nyahub",
    about = "NyaHub Android Build CLI",
    version,
    long_about = "CLI tool for building NyaHub Android APK with various options"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build Android APK
    Build {
        /// Build in debug mode (release is default)
        #[arg(short = 'D', long)]
        debug: bool,

        /// Clean build artifacts before building
        #[arg(short, long)]
        clean: bool,

        /// Install APK on connected device after build
        #[arg(short = 'I', long)]
        install: bool,

        /// Run the app after installing
        #[arg(short, long)]
        run: bool,

        /// Target ABI (arm64, armv7, i686, x86_64, or all)
        #[arg(short = 'a', long, default_value = "all")]
        abi: String,

        /// Build APK format (default)
        #[arg(long)]
        apk: bool,

        /// Build AAB format for Play Store
        #[arg(long)]
        aab: bool,

        /// Split APK per ABI
        #[arg(long)]
        split_per_abi: bool,

        /// Additional arguments to pass to tauri
        #[arg(trailing_var_arg = true)]
        extra_args: Vec<String>,
    },

    /// Install APK on connected device
    Install {
        /// Path to APK file
        #[arg(short, long)]
        apk: Option<String>,

        /// Install release APK (default)
        #[arg(short, long)]
        release: bool,

        /// Install debug APK
        #[arg(short, long)]
        debug: bool,
    },

    /// Run the app on connected device
    Run {
        /// Run release build (default)
        #[arg(short, long)]
        release: bool,

        /// Run debug build
        #[arg(short, long)]
        debug: bool,
    },

    /// Clean build artifacts
    Clean {
        /// Also clean Android build cache
        #[arg(short, long)]
        deep: bool,
    },

    /// Show build environment info
    Env,

    /// List connected Android devices
    Devices,
}

fn setup_env() -> Result<()> {
    // Force Java 21 (Gradle doesn't support Java 26+)
    std::env::set_var("JAVA_HOME", "/usr/lib/jvm/java-21-openjdk");
    std::env::set_var("ANDROID_HOME", "/opt/android-sdk");
    std::env::set_var("ANDROID_NDK_HOME", "/opt/android-sdk/ndk/27.0.12077973");

    let path = std::env::var("PATH").unwrap_or_default();
    let new_paths = vec![
        "/usr/lib/jvm/java-21-openjdk/bin",
        "/opt/android-sdk/cmdline-tools/latest/bin",
        "/opt/android-sdk/platform-tools",
        "/opt/android-sdk/ndk/27.0.12077973",
    ]
    .join(":");

    std::env::set_var("PATH", format!("{}:{}", new_paths, path));
    Ok(())
}

fn run_cmd(cmd: &str, args: &[String]) -> Result<i32> {
    println!(
        "{} {} {}",
        ">>".bright_blue().bold(),
        cmd.bright_green().bold(),
        args.join(" ").bright_white()
    );

    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute: {}", cmd))?;

    Ok(status.code().unwrap_or(1))
}

fn sign_apk(apk_path: &str) -> Result<String> {
    let keystore = std::env::var("HOME")
        .map(|h| format!("{}/.android/debug.keystore", h))
        .unwrap_or_else(|_| "/home/meme/.android/debug.keystore".into());

    if !std::path::Path::new(&keystore).exists() {
        println!("  {} Debug keystore not found at {}", "!".yellow(), keystore);
        return Ok(apk_path.to_string());
    }

    let zipaligned_path = apk_path.replace("-unsigned", "-zipaligned");
    let signed_path = apk_path.replace("-unsigned", "-signed");

    // Find zipalign
    let mut zipalign_path = String::new();
    let mut apksigner_path = String::new();
    if let Ok(android_home) = std::env::var("ANDROID_HOME") {
        // Find the latest build-tools version
        let build_tools_dir = format!("{}/build-tools", android_home);
        if let Ok(entries) = std::fs::read_dir(&build_tools_dir) {
            let mut versions: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                .filter(|s| s.chars().next().map(|c| c.is_numeric()).unwrap_or(false))
                .collect();
            versions.sort_by(|a, b| b.cmp(a)); // newest first
            if let Some(ver) = versions.first() {
                zipalign_path = format!("{}/{}/zipalign", build_tools_dir, ver);
                apksigner_path = format!("{}/{}/apksigner", build_tools_dir, ver);
            }
        }
    }

    // Step 1: zipalign before signing
    if std::path::Path::new(&zipalign_path).exists() {
        println!("{}", "Zipaligning APK...".bright_white().bold());
        let args = vec![
            "-f".into(),
            "4".into(),
            apk_path.into(),
            zipaligned_path.clone(),
        ];
        run_cmd(&zipalign_path, &args)?;
    } else {
        // Try system zipalign
        println!("{}", "Zipaligning APK (system)...".bright_white().bold());
        let args = vec![
            "-f".into(),
            "4".into(),
            apk_path.into(),
            zipaligned_path.clone(),
        ];
        let exit = run_cmd("zipalign", &args)?;
        if exit != 0 {
            println!("  {} zipalign not found, skipping", "!".yellow());
            std::fs::copy(apk_path, &zipaligned_path)?;
        }
    }

    // Step 2: Sign with apksigner (preferred) or jarsigner
    let apksigner_check = Command::new("which").arg("apksigner").status();

    let has_apksigner = std::path::Path::new(&apksigner_path).exists()
        || apksigner_check.map(|s| s.success()).unwrap_or(false);

    if has_apksigner {
        println!("{}", "Signing APK with apksigner...".bright_white().bold());
        let signer = if std::path::Path::new(&apksigner_path).exists() {
            apksigner_path.as_str()
        } else {
            "apksigner"
        };
        let args = vec![
            "sign".into(),
            "--ks".into(),
            keystore,
            "--ks-pass".into(),
            "pass:android".into(),
            "--key-pass".into(),
            "pass:android".into(),
            "--out".into(),
            signed_path.clone(),
            zipaligned_path.clone(),
        ];
        run_cmd(signer, &args)?;

        // Verify
        let verify_args = vec!["verify".into(), signed_path.clone()];
        let _ = run_cmd(signer, &verify_args);
    } else {
        println!("{}", "Signing APK with jarsigner...".bright_white().bold());
        // jarsigner signs in place
        let args = vec![
            "-keystore".into(),
            keystore,
            "-storepass".into(),
            "android".into(),
            zipaligned_path.clone(),
            "androiddebugkey".into(),
        ];
        run_cmd("jarsigner", &args)?;

        // jarsigner modifies in place, copy to signed path
        std::fs::copy(&zipaligned_path, &signed_path)?;

        println!("  {} (jarsigner signed in place)", "!".yellow());
    }

    Ok(signed_path)
}

fn check_env() -> Result<()> {
    let checks = [
        ("JAVA_HOME", "/usr/lib/jvm/java-21-openjdk"),
        ("ANDROID_HOME", "/opt/android-sdk"),
        ("ANDROID_NDK_HOME", "/opt/android-sdk/ndk/27.0.12077973"),
    ];

    let mut all_ok = true;

    for (name, expected) in &checks {
        match std::env::var(name) {
            Ok(val) => {
                if val == *expected {
                    println!("  {} {} = {}", "✓".green(), name, val.bright_cyan());
                } else {
                    println!(
                        "  {} {} = {} (expected: {})",
                        "!".yellow(),
                        name,
                        val.bright_cyan(),
                        expected
                    );
                }
            }
            Err(_) => {
                println!("  {} {} not set", "✗".red(), name);
                all_ok = false;
            }
        }
    }

    // Check for npx
    let npx_check = Command::new("which").arg("npx").status();
    match npx_check {
        Ok(status) if status.success() => {
            println!("  {} npx available", "✓".green());
        }
        _ => {
            println!("  {} npx not found", "✗".red());
            all_ok = false;
        }
    }

    // Check for Java
    let java_check = Command::new("java")
        .arg("-version")
        .stderr(std::process::Stdio::piped())
        .status();
    match java_check {
        Ok(status) if status.success() => {
            println!("  {} Java available", "✓".green());
        }
        _ => {
            println!("  {} Java not found or wrong version", "✗".red());
            all_ok = false;
        }
    }

    if !all_ok {
        println!(
            "\n{}",
            "Some environment checks failed. Build may fail.".yellow()
        );
    }

    Ok(())
}

fn get_apk_path(release: bool) -> String {
    let base = "src-tauri/gen/android/app/build/outputs/apk";
    if release {
        // Tauri generates universal unsigned APKs
        let paths = [
            format!("{}/universal/release/app-universal-release-unsigned.apk", base),
            format!("{}/release/app-release.apk", base),
            format!("{}/universal/debug/app-universal-debug.apk", base),
            format!("{}/debug/app-debug.apk", base),
        ];
        for p in &paths {
            if std::path::Path::new(p).exists() {
                return p.clone();
            }
        }
        paths[0].clone()
    } else {
        let paths = [
            format!("{}/universal/debug/app-universal-debug.apk", base),
            format!("{}/debug/app-debug.apk", base),
        ];
        for p in &paths {
            if std::path::Path::new(p).exists() {
                return p.clone();
            }
        }
        paths[0].clone()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {
            debug,
            clean,
            install,
            run,
            abi,
            apk,
            aab,
            split_per_abi,
            extra_args,
        } => {
            setup_env()?;

            let is_release = !*debug;

            println!("{}", "╔══════════════════════════════════╗".bright_blue());
            println!("{}", "║     NyaHub Android Builder       ║".bright_blue());
            println!("{}", "╚══════════════════════════════════╝".bright_blue());
            println!();

            // Print env info
            println!("{}", "Environment:".bright_white().bold());
            check_env()?;
            println!();

            // Clean if requested
            if *clean {
                println!("{}", "Cleaning build artifacts...".yellow());
                run_cmd("npx", &["tauri".into(), "android".into(), "clean".into()])?;
                println!();
            }

            // Build
            let mode = if is_release {
                "release".bright_red()
            } else {
                "debug".bright_yellow()
            };
            println!(
                "{}",
                format!("Building {} APK...", mode).bright_white().bold()
            );

            let mut args = vec![
                "tauri".into(),
                "android".into(),
                "build".into(),
            ];

            if *debug {
                args.push("--debug".into());
            }

            // Target ABI mapping
            if abi != "all" {
                let target = match abi.as_str() {
                    "arm64" | "arm64-v8a" => "aarch64",
                    "armv7" | "armeabi-v7a" => "armv7",
                    "i686" | "x86" => "i686",
                    "x86_64" => "x86_64",
                    other => other,
                };
                args.push("--target".into());
                args.push(target.into());
            }

            // APK/AAB format
            if *aab {
                args.push("--aab".into());
            } else if *apk || !*aab {
                args.push("--apk".into());
            }

            if *split_per_abi {
                args.push("--split-per-abi".into());
            }

            args.extend(extra_args.clone());

            let exit_code = run_cmd("npx", &args)?;

            if exit_code != 0 {
                println!("{}", "Build failed!".bright_red().bold());
                std::process::exit(exit_code);
            }

            println!();
            println!("{}", "Build successful!".bright_green().bold());

            // Sign the APK
            let apk_path = get_apk_path(is_release);
            let signed_path = sign_apk(&apk_path)?;

            // Install if requested
            if *install || *run {
                println!();
                println!(
                    "{}",
                    format!("Installing {}...", signed_path).bright_white().bold()
                );

                let install_args = vec!["install".into(), "-r".into(), signed_path.clone()];
                let exit_code = run_cmd("adb", &install_args)?;

                if exit_code != 0 {
                    println!("{}", "Install failed!".bright_red().bold());
                    std::process::exit(exit_code);
                }

                println!("{}", "Installed successfully!".bright_green().bold());
            }

            // Run if requested
            if *run {
                println!();
                println!("{}", "Starting app...".bright_white().bold());

                let run_args = vec![
                    "shell".into(),
                    "am".into(),
                    "start".into(),
                    "-n".into(),
                    "com.nyaland.desktop/.MainActivity".into(),
                ];
                run_cmd("adb", &run_args)?;
            }

            // Print APK location
            println!();
            println!(
                "{} {}",
                "APK:".bright_white().bold(),
                signed_path.bright_cyan()
            );
        }

        Commands::Install { apk, release, debug } => {
            setup_env()?;

            let is_release = *release && !*debug;
            let apk_path = apk.clone().unwrap_or_else(|| get_apk_path(is_release));

            println!(
                "{}",
                format!("Installing {}...", apk_path).bright_white().bold()
            );

            let args = vec!["install".into(), "-r".into(), apk_path];
            let exit_code = run_cmd("adb", &args)?;

            if exit_code != 0 {
                println!("{}", "Install failed!".bright_red().bold());
                std::process::exit(exit_code);
            }

            println!("{}", "Installed successfully!".bright_green().bold());
        }

        Commands::Run { release, debug } => {
            setup_env()?;

            let is_release = *release && !*debug;
            let apk_path = get_apk_path(is_release);

            // Install first
            println!(
                "{}",
                format!("Installing {}...", apk_path).bright_white().bold()
            );

            let install_args = vec!["install".into(), "-r".into(), apk_path];
            run_cmd("adb", &install_args)?;

            // Then run
            println!();
            println!("{}", "Starting app...".bright_white().bold());

            let run_args = vec![
                "shell".into(),
                "am".into(),
                "start".into(),
                "-n".into(),
                "com.nyaland.desktop/.MainActivity".into(),
            ];
            run_cmd("adb", &run_args)?;
        }

        Commands::Clean { deep } => {
            setup_env()?;

            println!("{}", "Cleaning build artifacts...".yellow());

            // Clean Tauri Android
            run_cmd("npx", &["tauri".into(), "android".into(), "clean".into()])?;

            if *deep {
                println!();
                println!("{}", "Deep cleaning...".yellow());

                // Clean Gradle cache
                let gradle_dir = std::path::Path::new("src-tauri/gen/android/.gradle");
                if gradle_dir.exists() {
                    std::fs::remove_dir_all(gradle_dir)?;
                    println!("  Removed .gradle cache");
                }

                // Clean build directory
                let build_dir = std::path::Path::new("src-tauri/gen/android/app/build");
                if build_dir.exists() {
                    std::fs::remove_dir_all(build_dir)?;
                    println!("  Removed app/build directory");
                }
            }

            println!("{}", "Clean complete!".bright_green().bold());
        }

        Commands::Env => {
            setup_env()?;

            println!("{}", "Build Environment:".bright_white().bold());
            println!();
            check_env()?;

            println!();
            println!("{}", "Android SDK:".bright_white().bold());

            // Check for SDK components
            let sdk_dir = std::path::Path::new("/opt/android-sdk");
            if sdk_dir.exists() {
                let components = [
                    "platform-tools",
                    "cmdline-tools",
                    "ndk/27.0.12077973",
                    "build-tools/36.0.0",
                    "platforms/android-36",
                ];

                for component in &components {
                    let path = sdk_dir.join(component);
                    if path.exists() {
                        println!("  {} {}", "✓".green(), component.bright_cyan());
                    } else {
                        println!("  {} {} (not installed)", "✗".red(), component);
                    }
                }
            }

            println!();
            println!("{}", "Rust Targets:".bright_white().bold());

            let output = Command::new("rustup")
                .arg("target")
                .arg("list")
                .arg("--installed")
                .output()
                .context("Failed to check Rust targets")?;

            let targets = String::from_utf8_lossy(&output.stdout);
            for target in targets.lines() {
                if target.contains("android") {
                    println!("  {} {}", "✓".green(), target.bright_cyan());
                }
            }
        }

        Commands::Devices => {
            setup_env()?;

            println!("{}", "Connected Android Devices:".bright_white().bold());
            println!();

            let output = Command::new("adb")
                .arg("devices")
                .output()
                .context("Failed to list devices")?;

            let devices_output = String::from_utf8_lossy(&output.stdout);
            let mut lines = devices_output.lines();

            // Skip header line
            if let Some(_header) = lines.next() {
                for line in lines {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split('\t').collect();
                        if parts.len() >= 2 {
                            let device = parts[0];
                            let status = parts[1];
                            let status_colored = match status {
                                "device" => status.bright_green(),
                                "offline" => status.bright_red(),
                                "unauthorized" => status.bright_yellow(),
                                _ => status.bright_white(),
                            };
                            println!(
                                "  {} ({})",
                                device.bright_cyan(),
                                status_colored
                            );
                        }
                    }
                }
            }

            println!();
            println!(
                "{}",
                "Tip: Use 'nyahub build --install --run' to build and run on device"
                    .bright_white()
                    .italic()
            );
        }
    }

    Ok(())
}
