use dual_shock4_controller::joystick::{DeviceInfo, Joystick};

// Analog inputs are jittery, we need a moving average of some size to smooth out the values
const WINDOW_SIZE: usize = 50; // Higher values are less responsive but less jittery

fn main() {
    let joystick = Joystick::new();
    let device_info = DeviceInfo {
        vid: 0x054c,
        pid: 0x09cc,
    };
    let device = joystick.connect(device_info).expect("can't find device!"); //
    let gamepad = joystick.get_gamepad();

    // Values for our moving weighted average
    // Non-rusty way; this can be improved
    // TODO also implement for the y axis and the other stick
    let mut index = 0;
    let mut sum: usize = 0;
    let mut readings: [usize; WINDOW_SIZE] = [0; WINDOW_SIZE];

    // Main event loop
    loop {
        // Get status of the controller
        let mut buf = [0u8; 64];
        device.read_timeout(&mut buf[..], 1000).unwrap();
        let state = gamepad.get_state(&buf);

        // Press X to quit
        if state.x_button.pressed {
            println!("× button was pressed, bye!");
            break;
        }
        let stick = state.stick.left_stick;

        // Remove the oldest entry from the sum
        sum = sum - readings[index];

        // Read the value from the controller
        let value = stick.x;

        // Add the newest reading to the window
        readings[index] = value as usize;

        // Add the newest reading to the sum
        sum += value as usize;

        // Increment index and wrap to the window size
        // TODO, this is prone to runtime errors
        index = (index + 1) % WINDOW_SIZE;

        // Get an average of the window, which is our smoothed value
        let averaged = sum / WINDOW_SIZE;
        println!("{averaged}");
    }
}
