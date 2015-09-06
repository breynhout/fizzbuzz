fn main() {
    for x in 1..31 {
    	if x %3 == 0 && x %5 == 0{
    		println!("this is the one where the two are combined");
    	}
    	else if x %3 == 0{
    		println!("FUZZ-TONED GUITAR");
    	}
    	else if x %5 == 0{
    		println!("Save the bees");
    	}
    	else{
    	println!("Last night I dreamt I went to Manderley again. Also: {}", x);
    	}
    }
}
