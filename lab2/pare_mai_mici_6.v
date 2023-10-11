module pare_mai_mici();
integer i;
initial begin
  for (i = 0; i < 26; i=i+1) begin
    if(i % 2 == 0) begin
      $display(i);
    end
  end
end
endmodule