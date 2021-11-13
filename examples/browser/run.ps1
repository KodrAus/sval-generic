Push-Location $PSScriptRoot

$ErrorActionPreference = 'Stop'

try {
    wasm-pack build --target web --release
    if ($LASTEXITCODE) { exit 1 }

    docker run --rm -it -v "$(pwd):/usr/share/nginx/html:ro" -p 8080:80 nginx
}
finally {
    Pop-Location
}
