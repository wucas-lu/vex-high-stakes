#region VEXcode Generated Robot Configuration
from vex import *
import urandom

# Brain should be defined by default
brain=Brain()

# Robot configuration code
left_motor_a = Motor(Ports.PORT19, GearSetting.RATIO_18_1, False)
left_motor_b = Motor(Ports.PORT17, GearSetting.RATIO_18_1, False)
left_drive_smart = MotorGroup(left_motor_a, left_motor_b)
right_motor_a = Motor(Ports.PORT20, GearSetting.RATIO_18_1, True)
right_motor_b = Motor(Ports.PORT18, GearSetting.RATIO_18_1, True)
right_drive_smart = MotorGroup(right_motor_a, right_motor_b)
drivetrain = DriveTrain(left_drive_smart, right_drive_smart, 319.19, 295, 40, MM, 1)
stake_motor = Motor(Ports.PORT3, GearSetting.RATIO_18_1, False)
intake_motor = Motor(Ports.PORT10, GearSetting.RATIO_18_1, False)
controller_1 = Controller(PRIMARY)
intake_motor2 = Motor(Ports.PORT11, GearSetting.RATIO_18_1, False)


# wait for rotation sensor to fully initialize
wait(30, MSEC)


# Make random actually random
def initializeRandomSeed():
    wait(100, MSEC)
    random = brain.battery.voltage(MV) + brain.battery.current(CurrentUnits.AMP) * 100 + brain.timer.system_high_res()
    urandom.seed(int(random))
      
# Set random seed 
initializeRandomSeed()


def play_vexcode_sound(sound_name):
    # Helper to make playing sounds from the V5 in VEXcode easier and
    # keeps the code cleaner by making it clear what is happening.
    print("VEXPlaySound:" + sound_name)
    wait(5, MSEC)

# add a small delay to make sure we don't print in the middle of the REPL header
wait(200, MSEC)
# clear the console to make sure we don't have the REPL in the console
print("\033[2J")



# define variables used for controlling motors based on controller inputs
controller_1_left_shoulder_control_motors_stopped = True
controller_1_up_down_buttons_control_motors_stopped = True
controller_1_x_b_buttons_control_motors_stopped = True
drivetrain_l_needs_to_be_stopped_controller_1 = False
drivetrain_r_needs_to_be_stopped_controller_1 = False

# define a task that will handle monitoring inputs from controller_1
def rc_auto_loop_function_controller_1():
    global drivetrain_l_needs_to_be_stopped_controller_1, drivetrain_r_needs_to_be_stopped_controller_1, controller_1_left_shoulder_control_motors_stopped, controller_1_up_down_buttons_control_motors_stopped, controller_1_x_b_buttons_control_motors_stopped, remote_control_code_enabled
    # process the controller input every 20 milliseconds
    # update the motors based on the input values
    while True:
        if remote_control_code_enabled:
            
            # calculate the drivetrain motor velocities from the controller joystick axies
            # left = axis3
            # right = axis2
            drivetrain_left_side_speed = controller_1.axis3.position()
            drivetrain_right_side_speed = controller_1.axis2.position()
            
            # check if the value is inside of the deadband range
            if drivetrain_left_side_speed < 5 and drivetrain_left_side_speed > -5:
                # check if the left motor has already been stopped
                if drivetrain_l_needs_to_be_stopped_controller_1:
                    # stop the left drive motor
                    left_drive_smart.stop()
                    # tell the code that the left motor has been stopped
                    drivetrain_l_needs_to_be_stopped_controller_1 = False
            else:
                # reset the toggle so that the deadband code knows to stop the left motor next
                # time the input is in the deadband range
                drivetrain_l_needs_to_be_stopped_controller_1 = True
            # check if the value is inside of the deadband range
            if drivetrain_right_side_speed < 5 and drivetrain_right_side_speed > -5:
                # check if the right motor has already been stopped
                if drivetrain_r_needs_to_be_stopped_controller_1:
                    # stop the right drive motor
                    right_drive_smart.stop()
                    # tell the code that the right motor has been stopped
                    drivetrain_r_needs_to_be_stopped_controller_1 = False
            else:
                # reset the toggle so that the deadband code knows to stop the right motor next
                # time the input is in the deadband range
                drivetrain_r_needs_to_be_stopped_controller_1 = True
            
            # only tell the left drive motor to spin if the values are not in the deadband range
            if drivetrain_l_needs_to_be_stopped_controller_1:
                left_drive_smart.set_velocity(drivetrain_left_side_speed, PERCENT)
                left_drive_smart.spin(FORWARD)
            # only tell the right drive motor to spin if the values are not in the deadband range
            if drivetrain_r_needs_to_be_stopped_controller_1:
                right_drive_smart.set_velocity(drivetrain_right_side_speed, PERCENT)
                right_drive_smart.spin(FORWARD)
            # check the buttonL1/buttonL2 status
            # to control stake_motor
            if controller_1.buttonL1.pressing():
                stake_motor.spin(FORWARD)
                controller_1_left_shoulder_control_motors_stopped = False
            elif controller_1.buttonL2.pressing():
                stake_motor.spin(REVERSE)
                controller_1_left_shoulder_control_motors_stopped = False
            elif not controller_1_left_shoulder_control_motors_stopped:
                stake_motor.stop()
                # set the toggle so that we don't constantly tell the motor to stop when
                # the buttons are released
                controller_1_left_shoulder_control_motors_stopped = True
            # check the buttonUp/buttonDown status
            # to control intake_motor
            if controller_1.buttonUp.pressing():
                intake_motor.spin(FORWARD)
                controller_1_up_down_buttons_control_motors_stopped = False
            elif controller_1.buttonDown.pressing():
                intake_motor.spin(REVERSE)
                controller_1_up_down_buttons_control_motors_stopped = False
            elif not controller_1_up_down_buttons_control_motors_stopped:
                intake_motor.stop()
                # set the toggle so that we don't constantly tell the motor to stop when
                # the buttons are released
                controller_1_up_down_buttons_control_motors_stopped = True
            # check the buttonX/buttonB status
            # to control intake_motor2
            if controller_1.buttonX.pressing():
                intake_motor2.spin(FORWARD)
                controller_1_x_b_buttons_control_motors_stopped = False
            elif controller_1.buttonB.pressing():
                intake_motor2.spin(REVERSE)
                controller_1_x_b_buttons_control_motors_stopped = False
            elif not controller_1_x_b_buttons_control_motors_stopped:
                intake_motor2.stop()
                # set the toggle so that we don't constantly tell the motor to stop when
                # the buttons are released
                controller_1_x_b_buttons_control_motors_stopped = True
        # wait before repeating the process
        wait(20, MSEC)

# define variable for remote controller enable/disable
remote_control_code_enabled = True

rc_auto_loop_thread_controller_1 = Thread(rc_auto_loop_function_controller_1)

#endregion VEXcode Generated Robot Configuration

# Hai likes Molly v1 by Team 1599W
from vex import *

STAKE_UNCLAMPED_DEGREES = 150
STAKE_CLAMPED_DEGREES = -70

stake_clamped = False
intake_spinning = False

def log(message):
    brain.screen.next_row()
    brain.screen.print("> " + message)
    controller_1.screen.next_row()
    controller_1.screen.print("> " + message)

def toggle_stake():
    global STAKE_UNCLAMPED_DEGREES
    global STAKE_CLAMPED_DEGREES
    global stake_clamped
    global log

    log("Toggling stake")

    stake_motor.stop()

    if stake_clamped:
        stake_motor.spin_to_position(STAKE_CLAMPED_DEGREES)
    else:
        stake_motor.spin_to_position(STAKE_UNCLAMPED_DEGREES)
        
    drivetrain.set_drive_velocity(100, PERCENT)
    drivetrain.set_turn_velocity(100, PERCENT)
    log("Clamping stake" if stake_clamped else "Unclamping stake")
    stake_clamped = not stake_clamped

def autonomous(): 
    global log
    global toggle_stake
    log("Starting autonomous control")

    drivetrain.turn(RIGHT, 35)
    drivetrain.drive_for(REVERSE, 3000, MM)
    drivetrain.drive_for(FORWARD, 500, MM)

    intake_motor.spin(FORWARD)
    intake_motor2.spin(FORWARD)

    wait(5, SECONDS)

    intake_motor.stop()
    intake_motor2.stop()

    drivetrain.turn(RIGHT, 180)
    drivetrain.drive_for(REVERSE, 2000, MM)
    drivetrain.turn(LEFT, 45)

    intake_motor.spin(FORWARD)
    intake_motor2.spin(FORWARD)

    drivetrain.drive_for(REVERSE, 500, MM)

    wait(5, SECONDS)

    intake_motor.stop()
    intake_motor2.stop()

    drivetrain.turn(RIGHT, 765)
    drivetrain.drive_for(REVERSE, 2500, MM)

    intake_motor.spin(FORWARD)
    intake_motor2.spin(FORWARD)

    wait(3, SECONDS)

    drivetrain.drive_for(FORWARD, 500, MM)
    drivetrain.turn(RIGHT, 720)
    
def driver_control():
    global intake_spinning
    global log
    log("Starting driver control")
    log("Press X to toggle stake")
    log("Hold right shoulder to spin intake")

    controller_1.buttonX.pressed(toggle_stake)
    toggle_stake()

    while True:
        wait(10, MSEC)
        if controller_1.buttonR2.pressing():
            intake_motor.spin(FORWARD)
            intake_motor2.spin(FORWARD)

            if not intake_spinning:
                log("Starting intake")
                intake_spinning = True
        else:
            intake_motor.stop()
            intake_motor2.stop()

            if intake_spinning:
                log("Stopping intake")
                intake_spinning = False

comp = Competition(driver_control, autonomous)

brain.screen.clear_screen()
controller_1.screen.clear_screen()

log("Starting Hai likes Molly v1")

drivetrain.set_drive_velocity(100, PERCENT)
drivetrain.set_turn_velocity(100, PERCENT)

intake_motor.set_velocity(100, PERCENT)
intake_motor2.set_velocity(75, PERCENT)

stake_motor.set_velocity(100, PERCENT)
stake_motor.spin_to_position(STAKE_UNCLAMPED_DEGREES)
