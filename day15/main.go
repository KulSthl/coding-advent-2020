// You can edit this code!
// Click here and start typing.
package main

import "fmt"
func main() {
	var const_data = []int{1,2,16,19,18,0};
	var const_test = []int{0,3,6};
	_ = const_test;
	_ = const_data;
	var memory = [][2]int{};
	var data = const_data;
	data, memory = initialize(data,memory)
	var _ = 0;
	var target = data[len(data)-1]
	var length = len(data);
	for i := 0; length < 2020; i++ {
		fmt.Println(i)
		 memory , _, target,length = playGame(data,memory,target,length)
		
		// fmt.Println(memory)
		// fmt.Println(data)
	}
	fmt.Println("Result: ",target)
	// fmt.Println(memory)
}
func playGame(data []int ,memory [][2]int,target int,length int)([][2]int, int,int,int){
	var spoken = 0;
	// var target = data[len(data)-1];
	var isInMemory = false;
	var previous = -1;
	for idx,integer := range (memory){
		if target == integer[1] {
			previous = integer[0];
			update := [2]int{length-1,target}
			memory[idx] = update
			isInMemory = true;
			break;
		}
	}
	if isInMemory {
		spoken := length-1-previous
		target = spoken;
		// data = append(data,spoken);
	} else {
		var temp = [2]int{length-1,target}
		memory = append(memory, temp);
		target = 0;
		// data = append(data, 0);
	}
	length += 1;
	return memory ,spoken,target,length
}
func initialize(data []int ,memory [][2]int)([]int, [][2]int){
	var mem = memory;
	for idx,integer := range data{
		var temp = [2]int{idx,integer}
		mem = append(mem,temp);
	}
	data = append(data,0);
	return data, mem;
}
