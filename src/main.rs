use serialport;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let serialports = &serialport::available_ports().expect("no serial port");
    for port in serialports {
        println!("{}", port.port_name);
    }
    // println!("{}", port_name);
    let mut s1 = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open serial port");

    s1.write("Hello".as_bytes())
        .expect("Failed to write to serial port");

    // Clone the port
    let mut clone = s1.try_clone().expect("Failed to clone");
    // Send out 4 bytes every second
    thread::spawn(move || loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();
        clone
            .write_all(message.as_bytes())
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(1000));
    });

    println!("Enter str");

    let mut rxbuf: [u8; 128] = [0; 128];
    loop {
        match s1.read(&mut rxbuf[..]) {
            Ok(n) => println!("The bytes: {:?}", &rxbuf[..n]),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        thread::sleep(Duration::from_millis(10));
    }
}
