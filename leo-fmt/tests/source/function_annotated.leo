program test.aleo{
  @test
    transition test_add( )
  {
return ;
    }


@test
      @should_fail
  transition test_fail(){
    return;
  }
}
