// adding multiline support for Leo
// all the code in this file should compile just fine

circuit BipBop {
    bip
    :
    u32,

    bop
    : u32,
    
    blup :
    u32,

    function gimme_five() -> Self {
        return Self {
            bip: 1,
            bop: 2,
            blup: 3
        }
    }

    function main() -> (
        u32
        ,
        u64
    ) {
        return (
            100
            ,
            1000
        )
    }
}

function main() -> (
    u32
    ,  
    u64
) {
    let 
    a 
    : u32 
    = 100
    ;

    let b : (
        u32, 
        u64
    ) = BipBop::gimme_five().main();

    console.assert(true);
} 

