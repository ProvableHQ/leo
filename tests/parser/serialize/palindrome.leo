/*
namespace: Serialize
expectation: Pass
*/

// This Program takes in any 20-byte low register string and tells 
// whether a string is a palindrome ignoring any spaces.

function main(str: [char; 20]) -> bool {
    return is_palindrome(str);
}

function is_palindrome(str: [char; 20]) -> bool {
    const str_len: u32 = 20u32; // saving const for convenience

    // By default we assume that input is a palindrome.
    let result: bool = true;
    let processed: u8 = 0u8;

    for start: u32 in 0u32..(str_len / 2u32) {
        let start_sym: char = str[start];
        if start_sym != ' ' {
            let skipped: u8 = 0u8;
            let end_empty: u8 = 0u8;
            let end_sym: char = ' ';

            for end: u32 in (str_len - 1u32)..start {
                if str[end] != ' ' && skipped == processed && end_sym == ' ' {
                    end_sym = str[end];
                } else {
                    end_empty = end_empty + 1u8;
                    if str[end] != ' ' {
                        skipped = skipped + 1u8;
                    }
                }
            }

            // If there are symbols left to the right from the start.
            if end_sym != ' ' {
                console.log("Comparing: {} ? {}", start_sym, end_sym);

                if result {
                    result = (start_sym == end_sym);
                }

                processed = processed + 1u8;
            }
        }
    }

    console.log("Result is: {}", result);

    return result;
}

@test
function test_is_palindrome() {
    console.assert(is_palindrome("a           b   a   "));
    console.assert(is_palindrome("😀😀😀😀😀          😀😀😀😀😀"));
    console.assert(is_palindrome("borrow or rob       "));
    console.assert(is_palindrome("bbbb aaaa  aaaa bbbb"));
    console.assert(is_palindrome("aaaaaaaaaaaaaaaaaaaa"));
    console.assert(is_palindrome("taco cat            "));
}
