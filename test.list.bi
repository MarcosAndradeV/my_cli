:i count 7
:b shell 22
cargo run -q --bin say
:i returncode 0
:b stdout 170
Usage: target/debug/say <COMMAND> [ARGS] [[-|--]FLAG]
COMMANDS:
    say <msg> [-d]          Prints the message.
    sayTo <msg> [-t <NAME>] Prints the message to <NAME>.

:b stderr 0

:b shell 35
cargo run -q --bin say -- say Hi -d
:i returncode 0
:b stdout 21
DEBUG: You say: "Hi"

:b stderr 0

:b shell 40
cargo run -q --bin say -- sayTo Hi -t Me
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

:b shell 29
cargo run -q --bin say -- aaa
:i returncode 0
:b stdout 170
Usage: target/debug/say <COMMAND> [ARGS] [[-|--]FLAG]
COMMANDS:
    say <msg> [-d]          Prints the message.
    sayTo <msg> [-t <NAME>] Prints the message to <NAME>.

:b stderr 32
ERROR: Unknown subcommand `aaa`

:b shell 32
cargo run -q --bin say -- say -d
:i returncode 0
:b stdout 20
ERROR: Expected msg

:b stderr 0

:b shell 34
cargo run -q --bin say -- sayTo Hi
:i returncode 0
:b stdout 170
Usage: target/debug/say <COMMAND> [ARGS] [[-|--]FLAG]
COMMANDS:
    say <msg> [-d]          Prints the message.
    sayTo <msg> [-t <NAME>] Prints the message to <NAME>.

:b stderr 36
ERROR: Missing required flags ["t"]

