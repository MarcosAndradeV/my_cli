:i count 7
:b shell 23
cargo run -q --bin test
:i returncode 0
:b stdout 171
Usage: target/debug/test <COMMAND> [ARGS] [[-|--]FLAG]
COMMANDS:
    say <msg> [-d]          Prints the message.
    sayTo <msg> [-t <NAME>] Prints the message to <NAME>.

:b stderr 0

:b shell 36
cargo run -q --bin test -- say Hi -d
:i returncode 0
:b stdout 21
DEBUG: You say: "Hi"

:b stderr 0

:b shell 41
cargo run -q --bin test -- sayTo Hi -t Me
:i returncode 0
:b stdout 21
You say: "Hi"
To: Me

:b stderr 0

:b shell 13
echo "ERRORS"
:i returncode 0
:b stdout 7
ERRORS

:b stderr 0

:b shell 30
cargo run -q --bin test -- aaa
:i returncode 255
:b stdout 0

:b stderr 32
ERROR: Unknown subcommand `aaa`

:b shell 33
cargo run -q --bin test -- say -d
:i returncode 0
:b stdout 20
ERROR: Expected msg

:b stderr 0

:b shell 35
cargo run -q --bin test -- sayTo Hi
:i returncode 255
:b stdout 0

:b stderr 37
ERROR: Missing required flags ["-t"]

