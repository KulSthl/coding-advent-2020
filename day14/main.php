
<?php
$handle = @fopen("data.txt", "r");
if ($handle) {
    $line_num = 0;
    $memory = [];
    $result_mem = [];
    while (($buffer = fgets($handle, 4096)) !== false) {
        if(strpos($buffer,"mask")!==false){
            $memory = [];
            $buffer = explode("mask = ",$buffer)[1];
            $pos = 0;
            $arr1 = str_split(trim($buffer));
            foreach (array_reverse($arr1) as $el){
                if($el !== "X") {
                    array_push($memory, [$pos, intval($el)]);
                }
                $pos+=1;
            }
            print_r($memory);
        }
        else{
            $bufferArr = explode("=",$buffer);
            $num = decbin(intval($bufferArr[1]));
            $num = sprintf( "%036s", decbin(intval($bufferArr[1])));
            $pos = 0;
            $output_num = [];
            foreach (array_reverse(str_split($num)) as $item){
                $output = $item;
                foreach ($memory as $mem){
                    if($mem[0]===$pos){
                        $output = $mem[1];
                    }
                }
                array_push($output_num,$output);
                $pos +=1;
            }
            $output_num = bindec(implode(array_reverse($output_num)));
            $reg = '/(?<=\[)(.*)(?=\])/';
            $matches = [];
            preg_match($reg,$bufferArr[0],$matches);
            $found = false;
            $temp_mem_arr = [];
            foreach ($result_mem as $mem){
                $temp_memory = $mem;
                if($mem[0]===$matches[0]){
                    $temp_memory[1] = $output_num;
                    $found = true;
                }
                array_push($temp_mem_arr, $temp_memory);
            }
            $result_mem = $temp_mem_arr;
            if($found === false){
                array_push($result_mem,[$matches[0],$output_num]);
            }
        }
        $line_num +=1;
    }
    print_r($result_mem);
    $sum = 0.0;
    foreach ($result_mem as $mem){
        $sum += $mem[1];
    }
    printf("\n %d \n",$sum);

    if (!feof($handle)) {
        echo "Fehler: unerwarteter fgets() Fehlschlag\n";
    }
    fclose($handle);
}
?>
