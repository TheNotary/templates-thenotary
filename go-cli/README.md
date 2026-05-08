# FooBar
[![Go Reference](https://pkg.go.dev/badge/FOO_GIT_REPO_PATH.svg)](https://pkg.go.dev/FOO_GIT_REPO_PATH)
[![Tests](FOO_GIT_REPO_URL/actions/workflows/build.yml/badge.svg)](FOO_GIT_REPO_URL/actions/workflows/build.yml)


TODO: Describe your project

## Install From Github

    go install FOO_GIT_REPO_PATH/cmd/foo-bar@latest

## Build/ Run from Source

    go run cmd/foo-bar/main.go
    go build -o foo-bar cmd/foo-bar/main.go
    ./foo-bar

## Run Tests

Go needs a test formatter so...

    go install github.com/mfridman/tparse@latest
    function gotest() {
      set -o pipefail
      go test "$@" ./... -json | tparse -all
    } 
    gotest

Run a specific test from a table-driven test suite:

    gotest -run '^TestBeautify/Outdents_else_in_ERB$' 

If you lose track of your coverage, cheat with:

    color_scheme="sed -i 's/: black/: white/g' c.html && sed -i 's/: rgb(44, 212, 149)/: rgb(107 182 77)/g' c.html"
    alias gocover="go test -coverprofile=c.out ./... && go tool cover -html=c.out -o c.html && eval \"${color_scheme}\" && open c.html"
    gocover


# Ecosystem Notes
## Run and Delete These Commands for Public Projects

## Initialize the Project
The command `go mod tidy` looks through your source code and installs any dependencies missing in `go.mod`.  Most IDEs will run this command upon save along with running formatters and linters.

    go mod init FOO_GIT_REPO_PATH
    go mod tidy

## Install / Update Dependencies

    go get ./...

## Distribution Troublshooting
Versioned releases are cut exclusively by their git tag in Go eliminated the need to create "bumps version to x.y.z" commits!

Go releases are cached by "the go module proxy" which strictly adheres to immutable releases.  Even the `@latest` tag can render the wrong package when doing pre-release git repo cleanup.

Also, local caches will exist after installing a package, so you may want to clean those up and delete any binaries left behind:

    go clean -cache -modcache
    rm "${GOPATH}/bin/foo-bar"
