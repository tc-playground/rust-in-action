function test_file_stream() {
    local here=`dirname "$0"`; SCRIPT_PATH=`eval "cd \"$SCRIPT_PATH\" && pwd"`
    cargo run pattern ${here}/src/main.rs
}

function test_stdin_stream() {
    local here=`dirname "$0"`; SCRIPT_PATH=`eval "cd \"$SCRIPT_PATH\" && pwd"`
    cat ${here}/src/main.rs | cargo run pattern
}

$@