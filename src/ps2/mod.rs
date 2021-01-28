/// This module handles PS/2 devices.
/// TODO doc

use crate::util::container::Box;
use crate::event;
use crate::io;
use crate::util;

/// The interrupt number for keyboard input events.
const KEYBOARD_INTERRUPT: u8 = 33;

/// TODO doc
const DATA_REGISTER: u16 = 0x60;
/// TODO doc
const STATUS_REGISTER: u16 = 0x64;
/// TODO doc
const COMMAND_REGISTER: u16 = 0x64;

/// The maximum number of attempts for sending a command to the PS/2 controller.
const MAX_ATTEMPTS: usize = 3;

/// TODO doc
const TEST_CONTROLLER_PASS: u8 = 0x55;
/// TODO doc
const TEST_CONTROLLER_FAIL: u8 = 0xfc;

/// TODO doc
const TEST_KEYBOARD_PASS: u8 = 0x00;
// TODO TEST_KEYBOARD_FAIL

/// TODO doc
const KEYBOARD_ACK: u8 = 0xfa;
/// TODO doc
const KEYBOARD_RESEND: u8 = 0xf4;

/// TODO doc
const LED_SCROLL_LOCK: u8 = 0b001;
/// TODO doc
const LED_NUMBER_LOCK: u8 = 0b010;
/// TODO doc
const LED_CAPS_LOCK: u8 = 0b100;

// TODO Turn commands and flags into constants.

/// Enumation of keyboard keys.
pub enum KeyboardKey {
	KeyEsc,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	Key0,
	KeyMinus,
	KeyEqual,
	KeyBackspace,
	KeyTab,
	KeyQ,
	KeyW,
	KeyE,
	KeyR,
	KeyT,
	KeyY,
	KeyU,
	KeyI,
	KeyO,
	KeyP,
	KeyOpenBrace,
	KeyCloseBrace,
	KeyEnter,
	KeyLeftControl,
	KeyA,
	KeyS,
	KeyD,
	KeyF,
	KeyG,
	KeyH,
	KeyJ,
	KeyK,
	KeyL,
	KeySemiColon,
	KeySingleQuote,
	KeyBackTick,
	KeyLeftShift,
	KeyBackslash,
	KeyZ,
	KeyX,
	KeyC,
	KeyV,
	KeyB,
	KeyN,
	KeyM,
	KeyComma,
	KeyDot,
	KeySlash,
	KeyRightShift,
	KeyKeypadStar,
	KeyLeftAlt,
	KeySpace,
	KeyCapsLock,
	KeyF1,
	KeyF2,
	KeyF3,
	KeyF4,
	KeyF5,
	KeyF6,
	KeyF7,
	KeyF8,
	KeyF9,
	KeyF10,
	KeyNumberLock,
	KeyScrollLock,
	KeyKeypad7,
	KeyKeypad8,
	KeyKeypad9,
	KeyKeypadMinus,
	KeyKeypad4,
	KeyKeypad5,
	KeyKeypad6,
	KeyKeypadPlus,
	KeyKeypad1,
	KeyKeypad2,
	KeyKeypad3,
	KeyKeypad0,
	KeyKeypadDot,
	KeyF11,
	KeyF12,
}

// TODO Turn into a map
/*static normal_keys = vec![
	(0x01, KeyEsc),
	(0x02, Key1),
	(0x03, Key2),
	(0x04, Key3),
	(0x05, Key4),
	(0x06, Key5),
	(0x07, Key6),
	(0x08, Key7),
	(0x09, Key8),
	(0x0a, Key9),
	(0x0b, Key0),
	(0x0c, KeyMinus),
	(0x0d, KeyEqual),
	(0x0e, KeyBackspace),
	(0x0f, KeyTab),
	(0x10, KeyQ),
	(0x11, KeyW),
	(0x12, KeyE),
	(0x13, KeyR),
	(0x14, KeyT),
	(0x15, KeyY),
	(0x16, KeyU),
	(0x17, KeyI),
	(0x18, KeyO),
	(0x19, KeyP),
	(0x1a, KeyOpenBrace),
	(0x1b, KeyCloseBrace),
	(0x1c, KeyEnter),
	(0x1d, KeyLeftControl),
	(0x1e, KeyA),
	(0x1f, KeyS),
	(0x20, KeyD),
	(0x21, KeyF),
	(0x22, KeyG),
	(0x23, KeyH),
	(0x24, KeyJ),
	(0x25, KeyK),
	(0x26, KeyL),
	(0x27, KeySemiColon),
	(0x28, KeySingleQuote),
	(0x29, KeyBackTick),
	(0x2a, KeyLeftShift),
	(0x2b, KeyBackslash),
	(0x2c, KeyZ),
	(0x2d, KeyX),
	(0x2e, KeyC),
	(0x2f, KeyV),
	(0x30, KeyB),
	(0x31, KeyN),
	(0x32, KeyM),
	(0x33, KeyComma),
	(0x34, KeyDot),
	(0x35, KeySlash),
	(0x36, KeyRightShift),
	(0x37, KeyKeypadStar),
	(0x38, KeyLeftAlt),
	(0x39, KeySpace),
	(0x3a, KeyCapsLock),
	(0x3b, KeyF1),
	(0x3c, KeyF2),
	(0x3d, KeyF3),
	(0x3e, KeyF4),
	(0x3f, KeyF5),
	(0x40, KeyF6),
	(0x41, KeyF7),
	(0x42, KeyF8),
	(0x43, KeyF9),
	(0x44, KeyF10),
	(0x45, KeyNumberLock),
	(0x46, KeyScrollLock),
	(0x47, KeyKeypad7),
	(0x48, KeyKeypad8),
	(0x49, KeyKeypad9),
	(0x4a, KeyKeypadMinus),
	(0x4b, KeyKeypad4),
	(0x4c, KeyKeypad5),
	(0x4d, KeyKeypad6),
	(0x4e, KeyKeypadPlus),
	(0x4f, KeyKeypad1),
	(0x50, KeyKeypad2),
	(0x51, KeyKeypad3),
	(0x52, KeyKeypad0),
	(0x53, KeyKeypadDot),
	(0x57, KeyF11),
	(0x58, KeyF12),
];*/

// TODO Special keys

/// Enumeration of keyboard actions.
pub enum KeyboardAction {
	/// The key was pressed.
	Pressed,
	/// The key was released.
	Released,
}

/// The callback handling keyboard inputs.
static mut KEYBOARD_CALLBACK: Option::<Box::<dyn FnMut(KeyboardKey, KeyboardAction)>> = None; // TODO Handle data race

/// Tells whether the PS/2 buffer is ready for reading.
fn can_read() -> bool {
	unsafe { // IO operation
		io::inb(STATUS_REGISTER) & 0b1 != 0
	}
}

/// Tells whether the PS/2 buffer is ready for writing.
fn can_write() -> bool {
	unsafe { // IO operation
		io::inb(STATUS_REGISTER) & 0b10 == 0
	}
}

/// Waits until the buffer is ready for reading.
fn wait_read() {
	while !can_read() {}
}

/// Waits until the buffer is ready for reading.
fn wait_write() {
	while !can_write() {}
}

/// Clears the PS/2 controller's buffer.
fn clear_buffer() {
	while can_read() {
		unsafe { // IO operation
			io::inb(DATA_REGISTER);
		}
	}
}

/// Sends the given data `data` to the keyboard.
fn keyboard_send(data: u8) -> Result::<(), ()> {
	let mut response = 0;

	for _ in 0..MAX_ATTEMPTS {
		wait_write();
		unsafe { // IO operation
			io::outb(DATA_REGISTER, data);
		}

		wait_read();
		response = unsafe { // IO operation
			io::inb(DATA_REGISTER)
		};
		if response == KEYBOARD_ACK {
			return Ok(());
		}
	}

	if response == KEYBOARD_ACK {
		Ok(())
	} else {
		Err(())
	}
}

/// Sends the given command `command` and returns the response.
fn send_command(command: u8, expected_response: u8) -> Result::<(), ()> {
	for _ in 0..MAX_ATTEMPTS {
		wait_write();
		unsafe { // IO operation
			io::outb(COMMAND_REGISTER, command);
		}

		wait_read();
		let response = unsafe { // IO operation
			io::inb(DATA_REGISTER)
		};
		if response == expected_response {
			return Ok(());
		}
	}
	Err(())
}

/// Disables PS/2 devices.
fn disable_devices() {
	wait_write();
	unsafe { // IO operation
		io::outb(COMMAND_REGISTER, 0xad);
	}

	wait_write();
	unsafe { // IO operation
		io::outb(COMMAND_REGISTER, 0xa7);
	}
}

/// Enables the keyboard device.
fn enable_keyboard() -> Result::<(), ()> {
	wait_write();
	unsafe { // IO operation
		io::outb(COMMAND_REGISTER, 0xae);
	}

	keyboard_send(0xf0)?;
	keyboard_send(1)?;
	keyboard_send(0xf3)?;
	keyboard_send(0)?;
	keyboard_send(0xf4)?;
	Ok(())
}

/// TODO doc
fn get_config_byte() -> u8 {
	wait_write();
	unsafe { // IO operation
		io::outb(COMMAND_REGISTER, 0x20);
	}

	wait_read();
	unsafe { // IO operation
		io::inb(DATA_REGISTER)
	}
}

/// TODO doc
fn set_config_byte(config: u8) {
	wait_write();
	unsafe { // IO operation
		io::outb(COMMAND_REGISTER, 0x60);
	}

	wait_write();
	unsafe { // IO operation
		io::outb(DATA_REGISTER, config);
	}
}

/// Tests the PS/2 controller.
fn test_controller() -> Result::<(), ()> {
	send_command(0xaa, TEST_CONTROLLER_PASS)
}

/// TODO doc
fn test_device() -> Result::<(), ()> {
	send_command(0xab, TEST_KEYBOARD_PASS)
}

/// Reads a keystroke and returns the associated key and action.
fn read_keystroke() -> (KeyboardKey, KeyboardAction) {
	let _keycode = unsafe {
		io::inb(DATA_REGISTER)
	};

	// TODO
	(KeyboardKey::KeyA, KeyboardAction::Pressed)
}

/// Keyboard input events callback.
struct KeyboardCallback_ {}

impl event::InterruptCallback for KeyboardCallback_ {
	fn is_enabled(&self) -> bool {
		true
	}

	fn call(&self, _id: u32, _code: u32, _regs: &util::Regs) {
		let callback = unsafe { // Access to global variable
			&mut KEYBOARD_CALLBACK
		};
		if let Some(l) = callback {
			while can_read() {
				let (key, action) = read_keystroke();
				l(key, action);
			}
		}
	}
}

/// Initializes the PS/2 driver.
pub fn init() -> Result::<(), ()> {
	// TODO Check if PS/2 controller is existing using ACPI

	// TODO Disable interrupts during init

	disable_devices();
	clear_buffer();

	set_config_byte(get_config_byte() & 0b10111100);

	test_controller()?;
	test_device()?;
	enable_keyboard()?;

	set_config_byte(get_config_byte() | 0b1);
	clear_buffer();

	event::register_callback(KEYBOARD_INTERRUPT, 0, KeyboardCallback_ {})?; // TODO Unregister when unloading module
	Ok(())
}

/// Sets the callback for keyboard actions.
pub fn set_keyboard_callback<F: 'static + FnMut(KeyboardKey, KeyboardAction)>(f: F) {
	unsafe { // Access to global variable
		KEYBOARD_CALLBACK = Some(Box::new(f).unwrap());
	}
}

// TODO LEDs state
