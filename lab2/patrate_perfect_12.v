module patrate_perfecte();
integer i;
initial begin
  for (i = 0; i <= 100; i=i+1) begin
    if ($pow(i, 2) <= 100) begin
      $display($pow(i,2));
    end
  end
end
endmodule