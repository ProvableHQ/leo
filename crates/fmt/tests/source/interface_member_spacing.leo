interface CounterInterface{
mapping counter :address=>u64;
storage owner :address;
fn increment()->u64;
record Receipt;
}

program test.aleo: CounterInterface{
mapping counter :address=>u64;
storage owner :address;
record Receipt{
owner: address,
}
fn increment()->u64{
return 1u64;
}
}
