module pare_mai_mici();
integer i;
initial begin
  for (i = 50; i >= 0; i=i-1) begin
    if(50 % i == 0) begin
      $display(i);
    end
  end
end
endmodule