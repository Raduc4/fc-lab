module pare_mai_mici();
integer i;
initial begin
  for (i = 0; i < 11; i=i+1) begin
      $display(i*3);
  end
end
endmodule