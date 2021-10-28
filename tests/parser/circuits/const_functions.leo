/*
namespace: Parse
expectation: Pass
*/

circuit X {
    x: u32
    const function x() { 
        return ();
    }
    const function x(self) { 
        return ();
    }
    const function c(const self) { 
        return ();
    }
    const function b(const self, x: u32) {
        return ();
    }
    const function b(const self, const x: u32) {
        return ();
    }
}
