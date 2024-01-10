import nested_example_layer_2.aleo;
import nested_example_layer_1.aleo;

program nested_example_layer_0.aleo;

function main:
    input r0 as u32.public;
    input r1 as u32.private;
    call nested_example_layer_1.aleo/external_function r0 r1 into r2;
    output r2 as u32.private;
