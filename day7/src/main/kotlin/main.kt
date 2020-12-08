import java.io.FileReader
import java.lang.Exception

val bag_list = mutableListOf<Bag>()
var count:Int = 0
fun main(args: Array<String>) {
    var file = FileReader("data.txt");
    var line = file.forEachLine {
        interpreter(it)
    }
    println("Interpreting Done")
//    searchForSpecific(bag_list,"shiny gold")
    var result = 0
    println("Calculating Part 2")
    val bag = openBag("shinygold");
    if (null !=bag){
        bagsInTarget(bag,1  );
    }
    println(count)
    execPart1(true)

}
class Bag constructor(val name: String, val weight:Int){
    var visited = false
    var next = mutableListOf<Bag>()
    var hasTarget = false
}
fun interpreter(input:String){
    fun searchInt(str:String): Int {
        val str = str.split(" ").toString()
        println(str)
        var number = ""
        str.forEach {
            if(it.isDigit()){
                number+=it
            }
        }
        try {
            return number.toInt()
        }catch (e:Exception){
            return 0
        }
    }
    fun getBag(str:String,num:Int): String {
        var str = str.split(" ")
        var result = ""
        str.forEach {
            if(it.isNotEmpty()){
                if(!it.equals(num.toString())){
                    if(!(it.contains("bags")  || it.contains("bag"))){
                        var res = it
                        if(it.endsWith(".")){
                            res = it.removeSuffix(".")
                        }
                        result+=res
                    }
                }
            }
        }
        return result
    }
    if(input.contains("contain")) {
        val pair = input.split("contain");
        val key = getBag(pair[0],0)
        val input = pair[1];
        val bags = input.split(",")
        val key_bag = Bag(key,0)
        bag_list.add(key_bag)
        bags.forEach {
            val value = it.trim()
            val number = searchInt(value)
            val name = getBag(value,number);
            if(name != "noother"){
                var bag = Bag(name,number)
                key_bag.next.add(bag)
            }

        }
    }
}
fun execPart1(bool:Boolean){
//    part 1
    println("Calculating Part 1")
    var result = 0;
    for ( i in 0..bag_list.size){
        for (bag in bag_list) {
            hasTarget(bag,"shinygold")
        }
    }
    for (bag in bag_list) {
        if(bag.hasTarget){
            println(bag.name)
            result +=1;
        }
    }
    println(result - 1)
}
fun hasTarget(bag:Bag, target:String) {
    if(!bag.hasTarget){
        if (bag.name == target){
            bag.hasTarget = true;
        }
        else{
            for (l_bag in bag.next){
                var t_bag = openBag(l_bag.name)
                if(t_bag!=null){
                    if(!t_bag.hasTarget){
                        hasTarget(t_bag,target)
                    }
                    else{
                        bag.hasTarget = true
                    }
                }
            }
        }
    }
}
fun bagsInTarget(bag:Bag,weight: Int){
    if(!bag.next.isEmpty()){
        for (l_bags in bag.next){
            var t_bag = openBag(l_bags.name)
            if(t_bag!=null){
                var lweight = l_bags.weight * weight;
                count += lweight
                bagsInTarget(t_bag,lweight);
            }
        }
    }
}
fun openBag(target:String): Bag? {
    for (bag in bag_list){
        if(bag.name == target) return bag
    }
    return  null
}
