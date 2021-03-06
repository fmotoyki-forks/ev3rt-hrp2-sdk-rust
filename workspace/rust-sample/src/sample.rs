use ev3::balancer::*;
use ev3::battery::*;
use ev3::button::*;
use ev3::ev3rt::*;
use ev3::lcd::*;
use ev3::led::*;
use ev3::motor::*;
use ev3::sensor::*;

pub fn ultrasonic_sample(ultrasonic: &SensorPort, tail_motor: &MotorPort, arm_motor: &MotorPort) {
	sensor_config(ultrasonic, &SensorType::UltraSonicSensor);

	motor_config(tail_motor, &MotorTypeT::MediumMotor);
	motor_config(arm_motor, &MotorTypeT::LargeMotor);

	reset_counts(tail_motor);
	reset_counts(arm_motor);

	let mut lefttail = true;
	let center = 30;
	let diff = 20;

	let mut power_base = 0;
	let mut armup = true;

	let upper = 30;
	let lower = 5;

	loop {
		lap_dly_tsk(10);

		let distance = ultrasonic_sensor_get_distance(ultrasonic);
		if distance < 70 {
			power_base = 200;
		} else {
			if power_base > 0 {
				power_base -= 1;
			}
		}

		let arm_angle = get_counts(arm_motor);
		if armup {
			if arm_angle < upper {
				set_power(arm_motor, power_base / 10);
			} else {
				armup = false;
			}
		} else {
			if arm_angle > lower {
				set_power(arm_motor, -1 * power_base / 15);
			} else {
				armup = true;
			}
		}

		let tail_angle = get_counts(tail_motor);

		if lefttail {
			if tail_angle < center + diff {
				set_power(tail_motor, power_base / 10);
			} else {
				lefttail = false;
			}
		} else {
			if tail_angle > center - diff {
				set_power(tail_motor, -1 * power_base / 10);
			} else {
				lefttail = true;
			}
		}

		set_led_color(&LEDColorT::LEDGreen);
	}
}
/// 倒立振子機能を実施するサンプル
/// 無限ループで倒立し続ける
pub fn balancer_sample(
	l_motor: &MotorPort,
	r_motor: &MotorPort,
	tail_motor: &MotorPort,
	touch: &SensorPort,
	gyro: &SensorPort,
) {
	// モータポートの初期化
	motor_config(l_motor, &MotorTypeT::LargeMotor);
	motor_config(r_motor, &MotorTypeT::LargeMotor);
	motor_config(tail_motor, &MotorTypeT::LargeMotor);

	// ジャイロセンサの初期化ト
	sensor_config(gyro, &SensorType::GyroSensor);

	// タッチセンサの初期化
	sensor_config(touch, &SensorType::TouchSensor);

	stop(l_motor, true);
	stop(r_motor, true);
	stop(tail_motor, true);

	// タッチセンサが押されるまで待機
	while !touch_sensor_is_pressed(touch) {
		lap_dly_tsk(4);
	}

	gyro_sensor_reset(gyro);
	reset_counts(l_motor);
	reset_counts(r_motor);

	balancer_init();

	let mut r_pwm: i8 = 0;
	let mut l_pwm: i8 = 0;

	// 尻尾を挙げてスタート
	rotate(&tail_motor, -45, 20, false);

	loop {
		lap_dly_tsk(4);

		let forward: f32 = 50.0;
		let turn: f32 = 0.0;
		let gyro_rate: f32 = gyro_sensor_get_rate(gyro) as f32;
		let gyro_offset: f32 = 0.0;
		let theta_m_l = get_counts(l_motor);
		let theta_m_r = get_counts(r_motor);
		let voltage: f32 = lap_battery_voltage_mv() as f32;

		balancer_control(
			forward,
			turn,
			gyro_rate,
			gyro_offset,
			theta_m_l,
			theta_m_r,
			voltage,
			&mut l_pwm,
			&mut r_pwm,
		);

		if l_pwm != 0 {
			set_power(l_motor, l_pwm as i32);
		} else {
			stop(l_motor, true);
		}
		if r_pwm != 0 {
			set_power(r_motor, r_pwm as i32);
		} else {
			stop(r_motor, true);
		}
	}
}

/// ジャイロセンサで取得した角速度と角度を表示するサンプル
/// 無限ループでLCDにジャイロセンサで取得した角速度と角度を表示し続ける
pub fn gyro_sample(gyro: SensorPort) {
	sensor_config(&gyro, &SensorType::GyroSensor);
	gyro_sensor_reset(&gyro);

	loop {
		lap_dly_tsk(100);
		lcd_clear(LCDColorT::EV3LCDWhite);

		let rate = gyro_sensor_get_rate(&gyro);
		let angle = gyro_sensor_get_angle(&gyro);

		draw_value("Rate\0", rate as i32, "rad/s\0", 0, 0);
		draw_value("Angle\0", angle as i32, "deg\0", 0, 15);
	}
}

/// カラーセンサの反射光の強さをLCDに出力するサンプル
/// 無限ループで反射光の強さをLCDに表示し続ける
pub fn color_sensor_reflect_sample(color_sensor_port: &SensorPort) {
	sensor_config(&color_sensor_port, &SensorType::ColorSensor);

	loop {
		let reflect = color_sensor_get_reflect(&color_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Reflect\0", reflect as i32, "%\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// カラーセンサのRGB生値のをLCDに出力するサンプル
/// 無限ループでRGBのRAW値をLCDに表示し続ける
pub fn color_sensor_raw_sample(color_sensor_port: SensorPort) {
	sensor_config(&color_sensor_port, &SensorType::ColorSensor);
	let mut rgb = RGBRaw {
		red: 0,
		green: 0,
		blue: 0,
	};
	loop {
		color_sensor_get_rgb_raw(&color_sensor_port, &mut rgb);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Red  \0", rgb.red as i32, "\0", 0, 0);
		draw_value("Green\0", rgb.green as i32, "\0", 0, 15);
		draw_value("Blue \0", rgb.blue as i32, "\0", 0, 30);

		lap_dly_tsk(100);
	}
}

/// タッチセンサの押下状態をLCDに出力するサンプル
/// 無限ループでタッチセンサの押下状態をLCDに表示し続ける
#[allow(dead_code)]
pub fn touch_sensor_sample(touch_sensor_port: SensorPort) {
	sensor_config(&touch_sensor_port, &SensorType::TouchSensor);

	loop {
		let pressed = touch_sensor_is_pressed(&touch_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Touch\0", pressed as i32, "-\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// 本体ボタンの押下状態とLEDの動作をサンプル
/// 無限ループで本体ボタンの押下状態に応じてLEDの点灯状態を制御する
#[allow(dead_code)]
pub fn button_led_sample() {
	loop {
		lap_dly_tsk(100);
		if is_pressed(&ButtonT::RightButton) {
			set_led_color(&LEDColorT::LEDRed);
		} else if is_pressed(&ButtonT::LeftButton) {
			set_led_color(&LEDColorT::LEDGreen);
		} else if is_pressed(&ButtonT::UpButton) {
			set_led_color(&LEDColorT::LEDOrange);
		} else {
			set_led_color(&LEDColorT::LEFOff);
		}
	}
}

/// バッテリ電圧と電流の状態を表示するサンプル
/// 無限ループでバッテリ電圧と電流をLCDに表示し続ける
#[allow(dead_code)]
pub fn battery_sample() {
	set_font(LCDFontT::EV3FontLarge);

	loop {
		lap_dly_tsk(100);

		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Volt\0", lap_battery_voltage_mv(), "V\0", 0, 0);
		draw_value("Curr\0", lap_battery_current_ma(), "mA\0", 0, 15);
	}
}

/// 本体ボタンの押下状態に応じてモータを回転させるサンプル
#[allow(dead_code)]
pub fn button_motor_sample() {
	motor_config(&MotorPort::EV3PortA, &MotorTypeT::LargeMotor);
	motor_config(&MotorPort::EV3PortB, &MotorTypeT::MediumMotor);

	loop {
		lap_dly_tsk(100);

		if is_pressed(&ButtonT::RightButton) {
			set_power(&MotorPort::EV3PortA, 50);
		} else if is_pressed(&ButtonT::LeftButton) {
			set_power(&MotorPort::EV3PortA, -50);
		} else if is_pressed(&ButtonT::UpButton) {
			set_power(&MotorPort::EV3PortB, 50);
		} else if is_pressed(&ButtonT::DownButton) {
			set_power(&MotorPort::EV3PortB, -50);
		} else {
			stop(&MotorPort::EV3PortA, false);
			stop(&MotorPort::EV3PortB, false);
		}
	}
}
