module in_base_2();
integer i;  
initial begin
  for (i = 10; i >= 0; i=i-1) begin
    $display("%b",i);
  end
end
endmodule