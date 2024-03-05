#ifndef _wrapper_h
#define _wrapper_h

#ifdef __cplusplus
extern "C" {
#endif

#define EXIT_HALT	0
#define EXIT_REBOOT	1

typedef void *circle_handle_t;

void circle_init (void);

circle_handle_t act_led_create (void);
void act_led_destroy (circle_handle_t handle);
void act_led_on (circle_handle_t handle);
void act_led_off (circle_handle_t handle);
void act_led_blink (circle_handle_t handle, unsigned count, unsigned msec_on, unsigned msec_off);

unsigned kernel_options_get_width (void);
unsigned kernel_options_get_height (void);
unsigned kernel_options_get_log_level (void);
int kernel_options_get_log_serial_device_num (void);

circle_handle_t screen_device_create (unsigned width, unsigned height, unsigned display_num);

enum serial_parity_t
{
	serial_parity_none,
	serial_parity_odd,
	serial_parity_even
};

circle_handle_t serial_device_create (unsigned baudrate, unsigned data_bits, unsigned stop_bits,
				      enum serial_parity_t parity, unsigned device_num);

int device_write (circle_handle_t handle, const void *buffer, unsigned long size);

void timer_simple_ms_delay (unsigned msec);

enum log_severity_t
{
	log_panic,
	log_error,
	log_warning,
	log_notice,
	log_debug
};
void logger_create (circle_handle_t target_handle, unsigned log_level);
void logger_write (const char *source, enum log_severity_t severity, const char *msg);

#ifdef __cplusplus
}
#endif

#endif
