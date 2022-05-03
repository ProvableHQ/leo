/*
namespace: ParseStatement
expectation: Pass
*/

{}

{
    return 0u8;
}

{{}}

{
    {
        return 0u8;
    }
}

{
    if x {
        return 0u8;
    }
}

