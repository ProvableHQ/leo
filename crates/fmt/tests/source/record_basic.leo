program test.aleo {

fn mint(  public amt  :  u64  )  ->  Token  {
    return  Token  {  owner  :  self.caller  ,  amount  :  amt  }  ;
}

record Token
  {
owner:address,

  amount :  u64 ,
    }
}
