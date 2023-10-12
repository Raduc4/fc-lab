module generate_10_numbers();
integer i;
initial begin
  for (i = 0; i < 10; i=i+1) begin
    $display(i+1);
    $display($realtime);
  end
end
endmodule