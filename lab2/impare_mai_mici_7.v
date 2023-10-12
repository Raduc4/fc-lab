module impare_mai_mici();
integer i;
initial begin
  for (i = 0; i < 27; i=i+1) begin
    if(i % 2 == 1) begin
      $display(i);
    end
  end
end
endmodule