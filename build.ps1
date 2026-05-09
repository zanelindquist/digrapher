$VERSION = (Select-String '^version = "(.*)"' Cargo.toml).Matches.Groups[1].Value
$IMAGE = "zanelindquist/digrapher"

Write-Host "Building Docker image..."

docker build -t "${IMAGE}:${VERSION}" .

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed."
    exit 1
}

Write-Host "Tagging latest..."

docker tag "${IMAGE}:${VERSION}" "${IMAGE}:latest"

Write-Host "Pushing version tag..."

docker push "${IMAGE}:${VERSION}"

Write-Host "Pushing latest tag..."

docker push "${IMAGE}:latest"

Write-Host "Done."