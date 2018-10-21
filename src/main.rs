fn is_prim(num: u32,primes: &[u32])-> bool {
	for j in primes{
		if num%*j as u32== 0{
			return false;
		}
	}
	return true;
}

fn main() { 
	let args: Vec<_> = std::env::args().skip(1).collect();
	let length = args[0].parse().unwrap();
	let mut primes = vec![2,3];
	let mut x = 6;
	while primes.len() < length{
		let ism = is_prim(x-1,&primes);
		if ism{
			primes.push(x-1);
			//println!("updated prime list: {:?}",x-1);
		}
		let isp = is_prim(x+1,&primes);
		if isp{
			primes.push(x+1);
			//println!("updated prime list: {:?}",x+1);
		}
		x +=6;
	}
	println!("The {} first prime numbers: \n{:?}",length,primes)

}
