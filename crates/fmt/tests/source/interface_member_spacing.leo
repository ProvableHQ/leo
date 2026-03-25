interface CounterInterface{
mapping counter :address=>u64;
storage owner :address;
fn increment()->u64;
record Receipt;
}
