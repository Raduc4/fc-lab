module timp();

initial begin
 $display("Timpul este", $time, "ns ", "or ", $realtime, "s"); 
//  Nu am gasit o solutie pentru asta,
//  logic ar fi sa ne dea un timestamp dar asta e
end
endmodule