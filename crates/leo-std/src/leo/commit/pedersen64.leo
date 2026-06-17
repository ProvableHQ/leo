// Pedersen commitments for inputs up to 64 bits.

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_bool_to_address(x: bool, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_bool_to_field(x: bool, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_bool_to_group(x: bool, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_u8_to_address(x: u8, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_u8_to_field(x: u8, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_u8_to_group(x: u8, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_u16_to_address(x: u16, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_u16_to_field(x: u16, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_u16_to_group(x: u16, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_u32_to_address(x: u32, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_u32_to_field(x: u32, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_u32_to_group(x: u32, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_i8_to_address(x: i8, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_i8_to_field(x: i8, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_i8_to_group(x: i8, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_i16_to_address(x: i16, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_i16_to_field(x: i16, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_i16_to_group(x: i16, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `address`.
fn commit_i32_to_address(x: i32, r: scalar) -> address {
    return _pedersen64_commit_to_address(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `field`.
fn commit_i32_to_field(x: i32, r: scalar) -> field {
    return _pedersen64_commit_to_field(x, r);
}

// Commits to `x` with Pedersen-64 under randomizer `r` and returns the commitment as `group`.
fn commit_i32_to_group(x: i32, r: scalar) -> group {
    return _pedersen64_commit_to_group(x, r);
}

