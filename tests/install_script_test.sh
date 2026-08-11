#!/bin/sh
set -eu

repository_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
installer="$repository_root/scripts/install-release.sh"
tar_command=$(command -v tar)
temporary_directory=$(mktemp -d)

cleanup() {
    rm -rf "$temporary_directory"
}
trap cleanup EXIT HUP INT TERM

fail() {
    printf '%s\n' "$1" >&2
    exit 1
}

make_fixture() {
    fixture_root="$1"
    system_name="$2"
    machine_name="$3"

    mkdir -p "$fixture_root/scripts" "$fixture_root/target/release" "$fixture_root/stubs" "$fixture_root/release"
    cp "$installer" "$fixture_root/scripts/install-release.sh"
    printf '%s\n' 'version = "9.8.7"' > "$fixture_root/herdr-plugin.toml"
    printf '%s\n' '#!/bin/sh' 'exit 0' > "$fixture_root/release/herdr-palette"
    chmod +x "$fixture_root/release/herdr-palette"

    cat > "$fixture_root/stubs/uname" <<EOF
#!/bin/sh
case "\$1" in
    -s) printf '%s\\n' '$system_name' ;;
    -m) printf '%s\\n' '$machine_name' ;;
    *) exit 1 ;;
esac
EOF
    chmod +x "$fixture_root/stubs/uname"

    cat > "$fixture_root/stubs/curl" <<'EOF'
#!/bin/sh
set -eu

printf '%s\n' "$@" > "$CURL_ARGUMENTS"
output=''
while [ "$#" -gt 0 ]; do
    if [ "$1" = '-o' ]; then
        output="$2"
        shift 2
    else
        shift
    fi
done

[ -n "$output" ]
"$TAR_COMMAND" -czf "$output" -C "$RELEASE_DIRECTORY" herdr-palette
EOF
    chmod +x "$fixture_root/stubs/curl"
}

assert_platform() {
    fixture_root="$1"
    expected_target="$2"

    (
        cd "$fixture_root"
        PATH="$fixture_root/stubs:$PATH" \
            CURL_ARGUMENTS="$fixture_root/curl-arguments" \
            RELEASE_DIRECTORY="$fixture_root/release" \
            TAR_COMMAND="$tar_command" \
            sh scripts/install-release.sh
    )

    [ -x "$fixture_root/target/release/herdr-palette" ] || fail "installer did not create an executable binary"
    grep -F "/v9.8.7/herdr-palette-$expected_target.tar.gz" "$fixture_root/curl-arguments" >/dev/null \
        || fail "installer selected the wrong release asset"
}

linux_fixture="$temporary_directory/linux"
make_fixture "$linux_fixture" Linux x86_64
assert_platform "$linux_fixture" x86_64-unknown-linux-gnu

darwin_fixture="$temporary_directory/darwin"
make_fixture "$darwin_fixture" Darwin arm64
assert_platform "$darwin_fixture" aarch64-apple-darwin

unsupported_fixture="$temporary_directory/unsupported"
make_fixture "$unsupported_fixture" Linux arm64
if (
    cd "$unsupported_fixture"
    PATH="$unsupported_fixture/stubs:$PATH" sh scripts/install-release.sh >stdout 2>stderr
); then
    fail 'unsupported platform unexpectedly succeeded'
fi
grep -F 'unsupported platform' "$unsupported_fixture/stderr" >/dev/null \
    || fail 'unsupported platform error was not reported'
