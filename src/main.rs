fn main() {
    for x in 1..31 {
    	if x %3 == 0 && x %5 == 0{
    		println!("fizzbuzz");
    	}
    	else if x %3 == 0{
    		println!("fizz");
    	}
    	else if x %5 == 0{
    		println!("buzz");
    	}
    	else{
    	println!("{}", x);
    	}
    }
}
