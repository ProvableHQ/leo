/*
[case]
program = "test.aleo"
function = "classify"
input = ["5u32"]
[case]
program = "test.aleo"
function = "classify"
input = ["15u32"]
[case]
program = "test.aleo"
function = "classify"
input = ["50u32"]
[case]
program = "test.aleo"
function = "classify"
input = ["100u32"]
*/

// --- library: thresholds --- //

const LOW: u32 = 10u32;
const MID: u32 = 25u32;
const HIGH: u32 = 75u32;

// --- Next Program --- //

// --- library: scores --- //

const LOW_SCORE: u32 = 1u32;
const MID_SCORE: u32 = thresholds::LOW + thresholds::MID;
const HIGH_SCORE: u32 = MID_SCORE + thresholds::HIGH;

// --- Next Program --- //

// Use library constants in conditional logic and assertions.

program test.aleo {
    fn classify(x: u32) -> u32 {
        let result: u32 = 0u32;

        if x < thresholds::LOW {
            // x < 10: low tier
            result = scores::LOW_SCORE;
        } else if x < thresholds::MID {
            // 10 <= x < 25: mid tier
            result = scores::MID_SCORE;
        } else if x < thresholds::HIGH {
            // 25 <= x < 75: high tier
            result = scores::HIGH_SCORE;
        } else {
            // x >= 75: max tier, return all thresholds summed
            result = thresholds::LOW + thresholds::MID + thresholds::HIGH;
        }

        // Verify the thresholds are ordered correctly (compile-time constants used at runtime).
        assert(thresholds::LOW < thresholds::MID);
        assert(thresholds::MID < thresholds::HIGH);

        return result;
    }

    @noupgrade
    constructor() {}
}
