# leo new hello-world

cd ./project/examples/pedersen-hash

export PEDERSEN_HASH_CONSTRAINTS=1542;

# line that we're searching for is:
# `Build Number of constraints - 1539`
export ACTUAL_CONSTRAINTS=$($LEO build | grep constraints | awk '{print $NF}')

# if else expression with only else block
[[ PEDERSEN_HASH_CONSTRAINTS -eq ACTUAL_CONSTRAINTS ]] || { 
    echo >&2 "Number of constraints for Pedersen Hash is not $PEDERSEN_HASH_CONSTRAINTS"; 
    echo >&2 "Real number of constraints is $ACTUAL_CONSTRAINTS";
    exit 1; 
}
