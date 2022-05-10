/*
namespace: Compile
expectation: Pass
*/

function main() -> group {
    const pos_element: group = 1group;
    const neg_element: group = -1group;

    const pair_x_pos: group = (1, _)group;
    const pair_x_neg: group = (-1, _)group;

    const pair_y_pos: group = (_, 1)group;
    const pair_y_neg: group = (_, -1)group;

    return pair_x_neg;
}