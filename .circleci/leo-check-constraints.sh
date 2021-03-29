# leo new hello-world

cd ./project/examples/pedersen-hash

export PEDERSEH_HASH_CONSTRAINTS=1539

# 1. build 
# 2. find lines with constraint number
# 3. find lines with $PEDERSEH_HASH_CONSTRAINTS
# 4. count lines
# 4.Er if result is 0 -> constraint number changed
# 4.Ok if result is 1 -> all good

[[ $($LEO build | grep "Number of constraints" | grep $PEDERSEH_HASH_CONSTRAINTS | wc -l) -eq 1 ]] || { 
    echo >&2 "Number of constraints for Pedersen Hash is not $PEDERSEN_HASH_CONSTRAINTS"; 
    exit 1; 
} 
