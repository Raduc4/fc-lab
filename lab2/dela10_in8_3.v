module in_base_8();
integer i;  
initial begin
  for (i = 10; i >= 0; i=i-1) begin
    $display("%o",i);
  end
end
endmodule