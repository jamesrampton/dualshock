use dual_shock4_controller::joystick::{DeviceInfo, Joystick};
use moving_avg::MovingAverage;

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

    let mut avergage_left_x = MovingAverage::<f32>::new(WINDOW_SIZE);

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

        let value = stick.x;

        println!("{}", avergage_left_x.feed(value as f32));
    }
}
