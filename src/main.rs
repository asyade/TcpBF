use std::net::TcpStream;
use std::env::args;

fn	main() {
	let argv: Vec<String> = args().collect();

	if argv.len() != 2 {
		println!("Usage: tcpbf [xx.xx.xx.xx]", );
		return ;
	}
	let addr = &argv[1];
	let mut port: u16 = 1;
	let mut result: Vec<u16> = vec![];
	while port < u16::max_value() {
		if let Ok(stream) = TcpStream::connect(format!("{}:{}", addr, port)) {
			result.push(port);
		}
		port += 1;
	}
	println!("Ports found : {}", result.len());
	for item in result.into_iter() {
		println!("{}", item);
	}
}