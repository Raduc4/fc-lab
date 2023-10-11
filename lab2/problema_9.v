module generate_10_numbers();
integer i;
integer pow_result;
initial begin
  for (i = 1; i < 15; i=i+1) begin
    pow_result = $pow(2, i);

    if(pow_result <= 9999) begin
      $display(pow_result);
    end 
  end
end
endmodule