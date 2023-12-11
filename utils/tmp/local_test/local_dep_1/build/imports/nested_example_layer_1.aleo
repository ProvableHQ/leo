import nested_example_layer_2.aleo;

program nested_example_layer_1.aleo;

function external_function:
    input r0 as u32.public;
    input r1 as u32.private;
    call nested_example_layer_2.aleo/external_nested_function r0 r1 into r2;
    output r2 as u32.private;
