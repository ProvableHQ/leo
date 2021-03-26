// Returns a tuple using a conditional "if" statement.
function tuple_conditional () -> (
    i64,
    i64
) {
    if true {
        return (1, 1)
    } else {
        return (2, 2)
    }
}

function main() {
    const t = tuple_conditional();
}