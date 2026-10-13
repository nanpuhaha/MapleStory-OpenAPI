# scripts/verify-release.ps1
# Release readiness verification script for MapleStory-OpenAPI-Rust

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "🚀 Verifying MapleStory-OpenAPI-Rust Release Readiness" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$ErrorActionPreference = "Stop"

function Run-Step($name, $cmd) {
    Write-Host "`n🔹 [Step] $name" -ForegroundColor Yellow
    Invoke-Expression $cmd
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed at: $name" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    Write-Host "✅ Passed: $name" -ForegroundColor Green
}

Run-Step "1. Formatting check" "cargo fmt --all -- --check"
Run-Step "2. Clippy zero-warnings check" "cargo clippy --all-targets --all-features -- -D warnings"
Run-Step "3. Workspace unit & integration tests" "cargo test --all --all-features"
Run-Step "4. Documentation tests" "cargo test --doc --all-features"
Run-Step "5. Examples compilation check" "cargo check --examples --all-features"
Run-Step "6. Crates.io package dry-run" "cargo publish --dry-run -p maplestory-openapi --allow-dirty"

Write-Host "`n🎉 All release readiness checks passed successfully!" -ForegroundColor Green
