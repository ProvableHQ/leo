// The 'message' main function.
circuit Message {
    first: field,
    second: field,
}

function main(m: Message) -> field {
    return m.first + m.second;
}
