use std::fs;
const SPELLING : &[(&str,&[&str])] = &[
    ("ping", &["pign", "png","PING"]),
    ("count", &["coutn", "cont", "COUNT"]),
    ("times", &["time", "tims", "TIMES"]),
    ("stop", &["stpo", "STOP"]),
    ("length", &["lenght","LENGTHs"]),
];
trait Command{
    fn get_name(&self) -> &str;
    fn exec(&mut self, args:&[&str]) -> Result<(), String>;
}
struct PingCommand;
impl Command for PingCommand{
    fn get_name(&self) -> &str{
        "ping"
    }
    fn exec(&mut self, args:&[&str]) -> Result<(),String>
    {
        if !args.is_empty(){
            return Err("Comanda ping nu primeste argumente".to_string());}
        println!("Pong!");
        Ok(())
    }
}
struct CountCommand;
impl Command for CountCommand{
    fn get_name(&self) -> &str{
        "count"
    }
     fn exec(&mut self, args:&[&str]) -> Result<(),String>{
        println!("counted {} args", args.len());
        Ok(())
    }
}
struct TimesCommand{
    count:u32,
}
impl Command for TimesCommand{
    fn get_name(&self) -> &str{
        "times"
    }
    fn exec(&mut self, _args:&[&str]) -> Result<(),String>{
        self.count += 1;
        println!("called {} times",self.count);
        Ok(()) // nu am implementat faptul ca times nu poate primi argumente, nu mi s-a parut necesar
    }
}
struct LengthCommand; //numara lungimea totala a argumentelor (spatiu nu se pune)
impl Command for LengthCommand{
    fn get_name(&self) -> &str{
        "length"
    }
    fn exec(&mut self, args:&[&str]) -> Result<(),String>
    {
        let mut total_len = 0;
        for arg in args{
            total_len += arg.len();}
        println!("Total length: {}",total_len);
        Ok(())
    }
}
struct Terminal{
    commands: Vec<Box<dyn Command>>,
}
impl Terminal{
     fn new() -> Terminal{
        Terminal{
        commands:Vec::new(),
    }
}
    fn register(&mut self, cmd: Box<dyn Command>){
        self.commands.push(cmd);
    }
    fn run(&mut self){
        let input = match fs::read_to_string("data/file.txt"){
            Ok(content) => content,
            Err(_) => {println!("Eroare nu se poate citi din fisierul dat");
            return;
         }
        };
        for line in input.lines(){
            let parts:Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty(){
                continue;
            }
            let command = parts[0];
            let args = &parts[1..];
            if command == "stop"{
                println!("Oprirea executiei...");
                break;
            }
            let mut found:bool = false;
            for cmd in &mut self.commands{
                if cmd.get_name() == command{
                    if let Err(e) = cmd.exec(args){
                        println!("Error: {}", e);
                    };
                    found = true;
                    break;
                } 
            }
            if !found {
              for (correct,misspell) in SPELLING{
                for miss in *misspell{
                    if *miss == command{
                        println!("Comanda necunoscuta '{}'. Ati vrut sa scrieti '{}'?",miss,correct);
                        break;
                        }
                    }
                }
            }
        }
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(LengthCommand {}));

    terminal.run();
}