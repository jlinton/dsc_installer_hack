extern crate serialport;

// no per the DSC, a good reference besides the IT-100 docs https://cms.dsc.com/download.php?t=1&id=16238
// is here https://github.com/taligentx/dscKeybusInterface
// the second link is a complete solution for people who don't want to mess with linux/etc
// there is also a C++ project called dsc-alarm, its quite old and apparently written for OSX but not a garbage language (although it's java like code)...
// https://sourceforge.net/p/dsc-alarm/code/HEAD/tree/

//use serialport;

// so this fits in the "my first rust program" category, so a lot of things are stripped if not needed, that means
// no fancy classes or closures, its imperative/procedural programming all the way down. Subject to change as needed of course

// its also a giant WTF in some ways because there are a ton of DSC interface programs on github/etc
// None though that I could find actually allow panel programming in any meaningful way, and for starters
// I need to brute-force the installer code on my security system, which isn't something that is easily done with
// the it-100 because its meant to be more of a keypad/keybus interface than a panel interface. Lucky most
// of the panels can be "programmed" via they keypads if you happen to be a computer and enjoy remembering where
// in the huge state tree you are enough to progress via a 6-15 digit key sequences.

//use std::fs::File;
use serialport::prelude::*;
use std::time::Duration;
use core::result::*;
use std::{thread, time};

mod it100resp;

/*fn lookserial(name:&str)
{
	let mut file = File::open(name)?;
	let mut data = String::new();
	file.read_to_string(&mut data);
	println!("hello there {}", data);
}*/

// convert a nibble to a ASCII hex char
fn to_hex(val:u8) -> u8
{
	let mut ret = val + 0x30;
	if ret > 0x39
	{
		ret += 0x07;
	}
	return ret;
}

// convert a ASCII hex char to a nibble
fn from_hex(val:u8) -> u8
{
	let mut ret = val - 0x30;
	if ret > 0x9
	{
		ret -= 7;
	}
	return ret;
}

// drop the checksum and cr/lf at the end of a message we are about to send
fn appendtrailer(command: &mut Vec<u8>)
{
	let mut sum:u8 = 0;

	for val in command.iter()
	{
		sum = sum.wrapping_add(*val);
	}
	// compute checksum
	command.push(to_hex((sum & 0xF0) >> 4));
	command.push(to_hex(sum & 0x0F));
	// add the cr/lf
	command.push(0x0d);
	command.push(0x0a);
}

// validate a message received from the IT-100
// return its response code if its valid
fn checkmsg(command: &mut Vec<u8>) -> Result<u32, String>
{
	let mut len = command.len();
	if len < 4
	{
		return Err(String::from("short message"));
	}

	len -=1;
	if command[len] != 0x0a
	{
		return Err(String::from("incorrect trailer"));
	}
	len -=1;
	if command[len] != 0x0d
	{
		return Err(String::from("incorrect trailer"));
	}
	len -=1;
	let csum = from_hex(command[len]) + (from_hex(command[len-1])<<4);
	len -=1;


	// OK check the checksum
	let mut sum:u8 = 0;
	for pos in 0..len
	{
		sum = sum.wrapping_add(command[pos]);
	}

	if sum != csum
	{
		return Err(String::from("incorrect csum"));
	}

	// at this point presumably the whole message is OK, although there
	// are probably conditions where the sum matches

	let mut ret:u32 = from_hex(command[0]) as u32 *100;
	ret += from_hex(command[1]) as u32 * 10;
	ret += from_hex(command[2]) as u32 * 1;

	return Ok(ret);
}

// convert an integer message type to a ascii IT-100 message
fn buildmess(cmd:u32,msg:&mut Vec<u8>)
{
	let d1:u8 = (cmd%10) as u8;
	let mcmd = cmd / 10;
	let d2:u8 = (mcmd%10) as u8;
	let d3:u8 = (mcmd / 10) as u8;

	msg.clear();
	msg.push(to_hex(d3));
	msg.push(to_hex(d2));
	msg.push(to_hex(d1));
}


// send a message with data
fn send_mess_data(cmd:u32, msg:&Vec<u8>, port:&mut dyn SerialPort)
{
	let mut outstrng:Vec<u8> = Vec::new();

	buildmess(cmd, &mut outstrng);
	for byte in msg
	{
		outstrng.push(*byte);
	}
	appendtrailer(&mut outstrng);
	let obytes = port.write(&outstrng).expect("write fail");
	println!("sent {} bytes {}{}",obytes,outstrng[3] as char,outstrng[4] as char);
}

// send a message with 0 bytes of data
// this copy into the middle of the message thing here probably isn't efficient but
// I don't imagine someone is going to be sending a lot of messages..
fn send_mess(cmd:u32, port:&mut dyn SerialPort)
{
	let dummy:Vec<u8> = Vec::new();
	send_mess_data(cmd, &dummy, port);
}

fn press_key(key:u8, port:&mut dyn SerialPort)
{
	let mut keyv:Vec<u8> = Vec::new();
	keyv.push(key);
	send_mess_data(70, &keyv, port);
	thread::sleep(time::Duration::from_millis(250));
}

// send the "*8code" installer sequence, originally I had a nice state machine
// sending keys, checking LED states, but it was to brittle when the random
// message didn't come in, or the panel went into he weeds.
//
// Turns out just sending each key with a hard-coded timer, and letting
// the kernel queue the resulting messages until we get around to reading
// them works better.
fn send_installer_sequence(installer_code:u32, port:&mut dyn SerialPort)
{
	let c1 = installer_code % 10;
	let mut code = installer_code / 10;
	let c2 = code % 10;
	code /= 10;
	let c3 = code % 10;
	code /= 10;

	press_key(b'*', &mut *port);
	press_key(b'8', &mut *port);
	press_key(to_hex(code as u8), port);
	press_key(to_hex(c3 as u8), port);
	press_key(to_hex(c2 as u8), port);
	press_key(to_hex(c1 as u8), port);
}

fn main()
{
	let mut settings: SerialPortSettings = Default::default();
	settings.baud_rate = 9600;
	//settings.baud_rate = 19200; //cranking the baud only makes us wait longer for the response..
	//settings.baud_rate = 57600;
	//settings.baud_rate = 115200;
	settings.timeout = Duration::from_millis(10);
	let mut port = serialport::open_with_settings("/dev/ttyUSB0", &settings).expect("unable to open port");


	// left over code used to change the baud rate
	//let mut outstrng:Vec<u8> = [b'0', b'8', b'0', b'0'].to_vec(); //9600
	//let mut outstrng:Vec<u8> = [b'0', b'8', b'0', b'1'].to_vec(); //19200
	//let mut outstrng:Vec<u8> = [b'0', b'8', b'0', b'3'].to_vec(); //57600
	//let mut outstrng:Vec<u8> = [b'0', b'8', b'0', b'4'].to_vec(); //115200
	//let outstrng:Vec<u8> = [b'1'].to_vec();
	//send_mess_data(80, &outstrng,&mut *port);

	send_mess(1, &mut *port);
	thread::sleep(time::Duration::from_millis(2000));

	let mut bump_installer = true; // if an error happens we will retry the current installer code sequence
	let mut installer_code:u32 = 0; //holds the installer code we are testing
	let mut send_poll = 0; //after a command sequence a number of responses will happen, then we stop getting messages, this is the loop count used to determine the panel has finished its sequences
	let mut lstate:it100resp::LedState =it100resp::LedState{ready: false, program: 0};

	// hammer the "#" key a couple times to get us out of any menus
	// or thats the theory, sometimes my panel goes into stupid mode
	// accepts commands but never does anything with them
	// when that happens, power cycle seems to be the only solution
	press_key(b'#', &mut *port);
	press_key(b'#', &mut *port);

	loop
	{
		let mut serial_data:[u8;32] = [0; 32];//will read no more than 128? = Vec::new();
		let mut cmd_result = Vec::<u8>::new(); // [b'0', b'0', b'1'].to_vec();
		cmd_result.clear();
		loop
		{
			match port.read(&mut serial_data)
			{
				 Ok(bytes) =>
				 {
					 for this_byte in serial_data[0..bytes].iter()
					 {
					 //	cmd_result.append(&mut serial_data[0..bytes].to_vec());
						cmd_result.push(*this_byte);
						if *this_byte == 0x0a
						{
							match checkmsg(&mut cmd_result)
							{
								Ok(resp_val) =>
								{
									//println!("got good response {}", resp_val);
									it100resp::parse_msg(resp_val, &mut lstate, &cmd_result);
									if lstate.program == 2 //flashing
									{
										println!("found code {}", installer_code);
										return;
									}
									if resp_val == 501 //command error (should enum these)
									{
										bump_installer = false;
									}
								}
								Err(msg) =>
								{
									println!("got bad message: {} -{:?}", msg, cmd_result );
									bump_installer = false;
								}
							}
							cmd_result.clear(); //start over
							send_poll = 0;
						}
					 }
				 }
				 Err(_e) =>
				 {
					if cmd_result.len() < 2
					{
						// timeout
						send_poll += 1;
						if send_poll == 100 //75 it short enough that sometimes the error code doesn't come in before we start the next code
						{
							// if an error happened retry old code
							if bump_installer == true
							{
								installer_code +=1;
								if installer_code == 1_0000
								{
									println!("sigh, not found!");
									return;
								}
							}
							bump_installer = true;

							send_installer_sequence(installer_code, &mut *port);

							send_poll = 0;
						}
					}
					else
					{
						println!("timeout with partial");
						bump_installer = false;
					}
				}
			}
		}
	}
}
