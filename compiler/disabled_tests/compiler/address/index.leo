/*
namespace: Compile
expectation: Pass
input_file: inputs/index.in
*/

function main (x: u32) -> bool {
    const y = [aleo1x0rh2cudq93fhukrsce8sgvcphddv4qs0clph64stpg0hstfds9qjvxcg6; 3];
    let z = y[x];

	return z == y[0];
}