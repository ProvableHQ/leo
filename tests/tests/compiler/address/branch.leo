/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    
    transition main (x: address, y: bool) -> bool {
        let z: address = aleo1fj982yqchhy973kz7e9jk6er7t6qd6jm9anplnlprem507w6lv9spwvfxx;
        if y {
            z = aleo16s003g206rjms5pm4ak48340f7y4z4dsskuqfrd2gvqz6umh2qfq7lajfp;
        }
    
        return z == aleo1drcl2g8zxhxjzjw63ajp067gzvl94am3z7m7wgrzmr2ecd5sdq8sy66l5k;
    }}
