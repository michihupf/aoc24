year := "2024"
url := "https://adventofcode.com/" + year

run day:
    (cd day{{day}}; cargo run)

create day:
    # create template package
    cargo new day{{day}}
    echo "aoclib = { path = '../aoclib' }" >> day{{day}}/Cargo.toml
    cp template.rs day{{day}}/src/main.rs
    # get input
    curl '{{url}}/day/{{day}}/input' -H 'Cookie: session='$(cat $HOME/.aocrc) > day{{day}}/input

open day:
    xdg-open {{url}}/day/{{day}}

