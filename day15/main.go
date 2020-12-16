// You can edit this code!
// Click here and start typing.
package main

import "fmt"
func main() {
	var const_data = []int{1,2,16,19,18,0};
	var const_test = []int{0,3,6};
	_ = const_test;
	_ = const_data;
	var memory = []int{};
	var data = const_data;
	data, memory = initialize(data,memory)
	var spoken = 0;
	for i := 0; len(data) < 30000000; i++ {
		fmt.Println(i)
		data , memory , spoken = playGame(data,memory)
	}
	fmt.Println(spoken)
	fmt.Println(data)
	// fmt.Println(memory)
}
func playGame(data []int ,memory []int)([]int,[]int, int){
	var spoken = 0;
	var target = data[len(data)-1];
	var isInMemory = false;
	for _,integer := range memory{
		if target == integer {
			isInMemory = true;
		}
	}
	if isInMemory {
		var previous = -1;
		var last = -1;
		for index,integer := range data{
			if integer == target {
				previous = last;
				last = index;
			}
		}	
		spoken := last-previous
		data = append(data,spoken);
	} else {
		memory = append(memory, target);
		data = append(data, 0);
	}
	return data ,memory ,spoken
}
func initialize(data []int ,memory []int)([]int, []int){
	var mem = memory;
	for _,integer := range data{
		mem = append(mem,integer);
	}
	data = append(data,0);
	return data, mem;
}
