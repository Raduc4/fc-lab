module fib();
integer i, f1,f2;
initial begin
  f1=0;
  f2=1;

  for(i=0;i<20;i=i+1) begin
    $display(f1);
    f1=f1+f2;
    f2=f1-f2;
  end
  
end
endmodule