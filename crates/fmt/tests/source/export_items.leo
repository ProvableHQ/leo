// Interface comment.
export interface Example{fn value()->u8;}
export const VALUE:u8=1u8;export const OTHER:u8=2u8;
// Generic struct comment.
export struct Item::[N:u32]{values:[u8;N],}export struct Plain{value:u8,}
export fn value()->u8{return VALUE;}export fn generic::[N:u32](value:[u8;N])->[u8;N]{return value;}
export final fn finalize_value(){}export final fn generic_final::[N:u32](value:[u8;N]){}
export view fn view_value()->u8{return VALUE;}export view fn generic_view::[N:u32](value:[u8;N])->[u8;N]{return value;}
@no_inline
export fn annotated_value()->u8{return OTHER;}
