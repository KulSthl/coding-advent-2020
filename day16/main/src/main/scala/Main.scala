import scala.io.Source
import scala.annotation.switch
import scala.collection.mutable.ListBuffer
object Main extends App {
  val filename = "test.txt";
  def run_part1(filename:String){
    var mode = 0;
    var errors = 0;
    var rules = new ListBuffer[Integer]();
    for (line <- Source.fromFile(filename).getLines) {
      if (line.startsWith("your")){
        mode = 1;
      }
      else if(line.startsWith("nearby")){
        mode = 2;
      }
      else if(line.isEmpty()){}
      else{
        if(mode == 0){
          interpreter_rules(line).foreach(f => {
            rules += f;
          });
        }
        else if (mode ==1 ){
          errors+= ticket_analyzer(line,rules.toList)._1
        }
        else if (mode ==  2){
            var local_error = ticket_analyzer(line,rules.toList)._1
            errors+= local_error;
        }
      }
    }
    printf("Result Part 1: %d\n",errors)
  }
  def run_part2(filename:String){
    var mode = 0
    var not_departure = new ListBuffer[Int]();
    var departure_rules = new ListBuffer[Int]();
    for (line <- Source.fromFile(filename).getLines) {
    if (line.startsWith("your")){
      mode = 1;
    }
    else{ 
    if (mode == 1){
      var index = 0
      var nmbr = 1.0
      line.trim().split(",").map(f => Integer.parseInt(f)).foreach(f => {
        if(!not_departure.contains(index)&&departure_rules.contains(f)){
          nmbr*=f
        }
        index += 1;
      })
      printf("\nResult 2: %f\n",nmbr)
    }
    mode = 2;
  }
  }
  }
  def interpreter_rules(line:String) : List[Integer]  = {
    var res = line.split(":")
    var numberArr = res(1).split("or");
    var output = new ListBuffer[Integer](); 
    numberArr.foreach(f => {
      getNumberRange(f).foreach(f => {
        output += f;
      })
    })
    return output.toList;
  }
  def getNumberRange(str:String):ListBuffer[Integer] = {
      var local_str = str.trim()
      var numberArr = local_str.split("-").map(f => Integer.parseInt(f))
      var output = new ListBuffer[Integer]();
      for( a <- numberArr(0) to numberArr(1)) {
        output+=a;
      }
      return output;
  }
  def ticket_analyzer(str:String,rules :List[Integer]):(Integer,ListBuffer[Int]) = {
    var errors = 0
    var number_arr = str.trim().split(",").map(f => Integer.parseInt(f));
    var index = 0;
    var index_arr = new ListBuffer[Int]()
    number_arr.foreach(f => {
      if(rules.contains(f)){

      }
      else{
        errors +=f;
        index_arr+=index;
      }
      index+=1;
    })
    return (errors,index_arr);
  }
  def interpreter_departure(line:String) : List[Integer] = {
    var output = new ListBuffer[Integer](); 
     if(line.startsWith("departure")){
          var res = line.split(":")
        var numberArr = res(1).split("or");
        numberArr.foreach(f => {
          getNumberRange(f).foreach(f => {
            output += f;
          })
        })
      }
      return output.toList;
     }
}