//use std::fmt;


struct IT100LedT<'a>
{
    led: u8,
    description: &'a str,
}

static IT100LED:[IT100LedT;10] = [
IT100LedT {
   led: b'0',
   description: "Dummy"
},
IT100LedT {
   led: b'1',
   description: "Ready"
},
IT100LedT {
   led: b'2',
   description: "Armed"
},
IT100LedT {
   led: b'3',
   description: "Memory"
},
IT100LedT {
   led: b'4',
   description: "Bypass"
},
IT100LedT {
   led: b'5',
   description: "Trouble"
},
IT100LedT {
   led: b'6',
   description: "Program"
},
IT100LedT {
   led: b'7',
   description: "Fire"
},
IT100LedT {
   led: b'8',
   description: "Backlight"
},
IT100LedT {
   led: b'9',
   description: "AC"
},
];

struct It100RespT<'a>
{
    cmd: u32,
	  data_len: usize,
	  description: &'a str,
    //formt: &'a str,
}

// hmm should i drop in some methods to handle some of these?
static IT100RESP:[It100RespT;84] = [ 
It100RespT {
  cmd: 500, 
  data_len: 3, 
  description: "Command accept"
},
It100RespT {
  cmd: 501, 
  data_len: 0, 
  description: "Command error"
},
It100RespT {
  cmd: 502, 
  data_len: 3, 
  description: "System error"
},


It100RespT {
  cmd: 550, 
  data_len: 10, 
  description: "Time broadcast"
},


It100RespT {
  cmd: 560, 
  data_len: 10, 
  description: "Ring detect"
},
It100RespT {
  cmd: 561, 
  data_len: 4, 
  description: "Indoor temperature"
},
It100RespT {
  cmd: 562, 
  data_len: 4, 
  description: "Outdoor temperature"
},
It100RespT {
  cmd: 563, 
  data_len: 8, 
  description: "Thermostat set point"
},


It100RespT {
  cmd: 570, 
  data_len: 35, 
  description: "Broadcast Labels"
},


It100RespT {
  cmd: 580, 
  data_len: 1, 
  description: "Baud rate set"
},


It100RespT {
  cmd: 601, 
  data_len: 4, 
  description: "Zone alarm"
},
It100RespT {
  cmd: 602, 
  data_len: 4, 
  description: "Zone alarm restore"
},
It100RespT {
  cmd: 603, 
  data_len: 4, 
  description: "Zone tamper"
},
It100RespT {
  cmd: 604, 
  data_len: 4, 
  description: "Zone tamper restore"
},
It100RespT {
  cmd: 605, 
  data_len: 3, 
  description: "Zone fault"
},
It100RespT {
  cmd: 606, 
  data_len: 3, 
  description: "Zone fault restore"
},
It100RespT {
  cmd: 609, 
  data_len: 3, 
  description: "Zone open"
},
It100RespT {
  cmd: 610, 
  data_len: 3, 
  description: "Zone open restore"
},


It100RespT {
  cmd: 620, 
  data_len: 4, 
  description: "Duress"
},
It100RespT {
  cmd: 621, 
  data_len: 0, 
  description: "Fire keypress"
},
It100RespT {
  cmd: 622, 
  data_len: 0, 
  description: "Fire keypress restore"
},
It100RespT {
  cmd: 623, 
  data_len: 0, 
  description: "Aux keypress"
},
It100RespT {
  cmd: 624, 
  data_len: 0, 
  description: "Aux keypress restore"
},
It100RespT {
  cmd: 625, 
  data_len: 0, 
  description: "Panic keypress"
},
It100RespT {
  cmd: 626, 
  data_len: 0, 
  description: "Panic keypress restore"
},


It100RespT {
  cmd: 631, 
  data_len: 0, 
  description: "Aux input keypress"
},
It100RespT {
  cmd: 632,
  data_len: 0, 
  description: "Aux input restore"
},


It100RespT {
  cmd: 650, //? this doesn't look right in the docs
  data_len: 1, 
  description: "Partition ready"
},
It100RespT {
  cmd: 651,
  data_len: 1, 
  description: "Partition not ready"
},
It100RespT {
  cmd: 652,
  data_len: 2, 
  description: "Partition armed"
},
It100RespT {
  cmd: 653, //? this doesn't look right in the docs
  data_len: 1, 
  description: "Partition ready to force"
},
It100RespT {
  cmd: 654, 
  data_len: 1, 
  description: "Partition ready"
},
It100RespT {
  cmd: 655, 
  data_len: 1, 
  description: "Partition disarm"
},
It100RespT {
  cmd: 656, 
  data_len: 1, 
  description: "Partition exit delay in progress"
},
It100RespT {
  cmd: 657, 
  data_len: 1, 
  description: "Partition entry delay in progress"
},
It100RespT {
  cmd: 658, 
  data_len: 1, 
  description: "Partition keypad in lockout"
},
It100RespT {
  cmd: 659, 
  data_len: 1, 
  description: "Partition keypad blank"
},
It100RespT {
  cmd: 660, 
  data_len: 1, 
  description: "Partition command output mode"
},


It100RespT {
  cmd: 670, 
  data_len: 1, 
  description: "Invalid access code"
},
It100RespT {
  cmd: 671, 
  data_len: 1, 
  description: "Function not available"
},
It100RespT {
  cmd: 672, 
  data_len: 1, 
  description: "Partition failed to arm"
},
It100RespT {
  cmd: 673, 
  data_len: 1, 
  description: "Partition busy"
},


It100RespT {
  cmd: 700, 
  data_len: 5, 
  description: "User armed"
},
It100RespT {
  cmd: 701, 
  data_len: 1, 
  description: "Special armed"
},
It100RespT {
  cmd: 702, 
  data_len: 1, 
  description: "Partial arming"
},


It100RespT {
  cmd: 750, 
  data_len: 5, 
  description: "User disarm"
},
It100RespT {
  cmd: 751, 
  data_len: 1, 
  description: "Special disarm"
},


It100RespT {
  cmd: 800, 
  data_len: 0, 
  description: "Check panel battery"
},
It100RespT {
  cmd: 801, 
  data_len: 0, 
  description: "Panel battery OK"
},
It100RespT {
  cmd: 802, 
  data_len: 0, 
  description: "Check panel AC"
},
It100RespT {
  cmd: 803, 
  data_len: 0, 
  description: "Panel AC OK"
},
It100RespT {
  cmd: 806, 
  data_len: 0, 
  description: "Check Bell"
},
It100RespT {
  cmd: 807, 
  data_len: 0, 
  description: "Bell OK"
},


It100RespT {
  cmd: 810, 
  data_len: 0, 
  description: "Check telephone 1"
},
It100RespT {
  cmd: 811, 
  data_len: 0, 
  description: "Telephone 1 OK"
},
It100RespT {
  cmd: 812, 
  data_len: 0, 
  description: "Check telephone 2"
},
It100RespT {
  cmd: 813, 
  data_len: 0, 
  description: "Telephone 2 OK"
},
It100RespT {
  cmd: 814, 
  data_len: 0, 
  description: "Check monitoring"
},
It100RespT {
  cmd: 815, 
  data_len: 0, 
  description: "System log nearly full"
},


It100RespT {
  cmd: 821, 
  data_len: 3, 
  description: "Zone low battery"
},
It100RespT {
  cmd: 822, 
  data_len: 3, 
  description: "Zone battery OK"
},
It100RespT {
  cmd: 825, 
  data_len: 3, 
  description: "Wireless key low battery"
},
It100RespT {
  cmd: 826, 
  data_len: 3, 
  description: "Wireless key battery OK"
},
It100RespT {
  cmd: 827, 
  data_len: 3, 
  description: "Wireless keypad low battery"
},
It100RespT {
  cmd: 828, 
  data_len: 3, 
  description: "Wireless keypad battery OK"
},
It100RespT {
  cmd: 829, 
  data_len: 0, 
  description: "Tamper alarm"
},
It100RespT {
  cmd: 830, 
  data_len: 0, 
  description: "Tamper alarm OK"
},
It100RespT {
  cmd: 831, 
  data_len: 0, 
  description: "Home automation error"
},
It100RespT {
  cmd: 832, 
  data_len: 0, 
  description: "Home automation OK"
},


It100RespT {
  cmd: 840, 
  data_len: 1, 
  description: "Partition trouble"
},
It100RespT {
  cmd: 841, 
  data_len: 1, 
  description: "Partition trouble cleared"
},
It100RespT {
  cmd: 842, 
  data_len: 0, 
  description: "Fire trouble"
},
It100RespT {
  cmd: 843, 
  data_len: 0, 
  description: "Fire trouble cleared"
},

It100RespT {
  cmd: 896, 
  data_len: 0, 
  description: "Keybus trouble"
},
It100RespT {
  cmd: 897, //https://github.com/SolidElectronics/evl-emu/blob/master/evl-emu.py
  data_len: 0, 
  description: "Keybus OK"
},

It100RespT {
  cmd: 900, 
  data_len: 2, 
  description: "Code required"
},
It100RespT {
  cmd: 901, 
  data_len: 37, //up to 6-37 bytes, this is one of the special variable sized ones
  description: "LCD update"
},
It100RespT {
  cmd: 902, 
  data_len: 4, 
  description: "LCD cursor"
},
It100RespT {
  cmd: 903, 
  data_len: 2, 
  description: "LED status"
},
It100RespT {
  cmd: 904, 
  data_len: 3, 
  description: "Beeps"
},
It100RespT {
  cmd: 905, 
  data_len: 4, 
  description: "Tone"
},
It100RespT {
  cmd: 906, 
  data_len: 3, 
  description: "Buzzer"
},
It100RespT {
  cmd: 907, 
  data_len: 0, 
  description: "Chime"
},
It100RespT {
  cmd: 908, 
  data_len: 6, 
  description: "Software version"
},
];

// the dsc leds are tristate, off/on/flashing
pub struct LedState
{
    pub ready: bool,
    pub program: u8,
}

fn show(bs: &[u8]) -> String 
{
    String::from_utf8_lossy(bs).into_owned()
}

pub fn parse_msg(resp: u32, lstate:&mut LedState,command: &Vec<u8>)
{
    // special case a couple status commands
    if resp == 903
    {
        let led = command[3]-0x30;
        if led+0x30 == IT100LED[led as usize].led
        {
            match command[4]{
              b'0' => println!("LED {}-OFF",IT100LED[led as usize].description),
              b'1' => println!("LED {}-ON",IT100LED[led as usize].description),
              b'2' => println!("LED {}-FLASH",IT100LED[led as usize].description),
              _ => println!("LED {}-{}",IT100LED[led as usize].description,command[4]),
            }
        }
        if led == 1 // ready
        {
            if command[4] == b'1' || command[4] == b'2' 
            {
                lstate.ready = true;
            }
            else 
            {
                lstate.ready = false;  
            }
        }
        if led == 6 // program
        {
            lstate.program = command[4] - 0x30;
        }
        return;
    }

    if resp == 610
    {
        return; //cause im tired of them
    }

    for cmd_index in IT100RESP.iter()
    {
        if resp == cmd_index.cmd
        {
            //let pretty = format!("{}cmd_index.description, command[3], command[4] )?;
            match cmd_index.data_len
            {
                1 => println!("got response {}-{}- {}", resp, cmd_index.description, command[3] as char),
                2 => println!("got response {}-{}- {}{}", resp, cmd_index.description, command[3] as char, command[4] as char),
                3 => println!("got response {}-{}- {}{}{}", resp, cmd_index.description, command[3] as char, command[4] as char, command[5] as char),
                4 => println!("got response {}-{}- {}{}{}{}", resp, cmd_index.description, command[3] as char, command[4] as char, command[5] as char, command[6] as char),
                37 => println!("got response {}-{}- {}", resp, cmd_index.description, show(&command[8..39])), 
                _ => println!("got response {}-{}, datalen {}", resp, cmd_index.description, cmd_index.data_len),
            }
            if command.len() - 7 != cmd_index.data_len
            {
                println!("Cmd len mismatch {}", command.len()-7);
            }
            return ;
        }
    }    
    println!("Cmd {} not found", resp);
}

