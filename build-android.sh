#!/bin/bash
# Build script for NyaHub Android APK
# Requires: Java 21, Android SDK, Android NDK, Rust Android targets

set -e

# Force Java 21 (Gradle doesn't support Java 26)
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk
export ANDROID_HOME=/opt/android-sdk
export ANDROID_NDK_HOME=/opt/android-sdk/ndk/27.0.12077973
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_NDK_HOME:$PATH"

echo "Using Java:"
java -version 2>&1

echo ""
echo "Building Android APK..."
npx tauri android build "$@"
