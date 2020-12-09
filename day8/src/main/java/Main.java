import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    class Command {
        public String action;
        public int number;
        public String sign;
        public boolean run = false;
        public boolean changed = false;
        public Command(String action, String sign, int number){
            this.action = action;
            this.sign = sign;
            this.number = number;
        }

        @Override
        public String toString() {
            return action+"/"+sign+"/"+number;
        }
    }
    ArrayList<Command> commandList = new ArrayList<Command>();
    public int calc(int value ,String sign, int number){
        switch (sign){
            case "+":
                return value+=number;

            case "*":
                return value*=number;
            case "-":
                return value-=number;
            case "/":
                return value/=number;
        }
        return value;
    }
    public ArrayList<Command> reset(){
        commandList.forEach(command -> {
            command.run = false;
        });
        return commandList;
    }
    public Command change(Command command){
        if(command.action.equals("nop")){
            command.action = "jmp";
        }
        else if (command.action.equals("jmp")){
            command.action = "nop";
        }
        return command;
    }
    public ArrayList<Command> init(boolean foundChanged){
        if(!foundChanged){
            for(var command : commandList){
                if(command.action.equals("nop")||command.action.equals("jmp")){
                    command.action = change(command).action;
                    command.changed = true;
                    break;
                }
            };
        }
        return commandList;
    }
    public ArrayList<Command> testNext(){
        boolean foundChange = false;
        for(var command : commandList){
            if(command.action.equals("nop")||command.action.equals("jmp")){
                if(!foundChange){
                    if(command.changed){
                        foundChange = true;
                        command.action = change(command).action;
                        command.changed = false;
                    }
                }
                else{
                    command.action = change(command).action;
                    command.changed = true;
                    break;
                }
            }
        };
        init(foundChange);
        return commandList;
    }
    public void interpret(){
        var value = 0;
        for (int i = 0;i < commandList.size();i++){
            boolean run = true;
            Command command = null;
            if(i >= commandList.size()){
                run = false;

            }
            else{
                command = commandList.get(i);
            }

            if(!command.run && run){
                switch (command.action){
                    case "acc":
                        value = calc(value,command.sign,command.number);
                        break;
                    case "jmp":
                        i = calc(i,command.sign,command.number)-1;
                        break;
                    case "nop":
                        break;
                }
                command.run = true;
            }
            else{
                System.out.println(value);
                reset();
                testNext();
                interpret();
                return;
            }
        }
        System.out.println("SUCCESS");
        System.out.println(value);
    }
    public Command getCommand(String line){
        var keyword = line.split(" ")[0];
        if(line.startsWith(keyword)){
            var positon = line.indexOf(keyword)+keyword.length()+1;
            var str = line.substring(positon).trim();
            return new Command(keyword,str.substring(0,1),Integer.parseInt(str.substring(1)));
        }
        return null;
    }
    public void readFile(){
        try {
            File file = new File("data.txt");
            Scanner sc = new Scanner(file);

            while (sc.hasNextLine())
                commandList.add(getCommand(sc.nextLine()));
        }
        catch (Exception err){
            err.printStackTrace();
        }
    }
    public static void main(String[] args) {
        Main main = new Main();
        main.readFile();
        main.interpret();
        System.out.println("Hello");
    }

}