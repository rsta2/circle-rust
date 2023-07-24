#ifndef _wrapper_h
#define _wrapper_h

#ifdef __cplusplus
extern "C" {
#endif

#define EXIT_HALT	0
#define EXIT_REBOOT	1

typedef void *circle_handle_t;

void circle_init ();

circle_handle_t act_led_create (void);
void act_led_destroy (circle_handle_t handle);
void act_led_on (circle_handle_t handle);
void act_led_off (circle_handle_t handle);
void act_led_blink (circle_handle_t handle, unsigned count, unsigned msec_on, unsigned msec_off);

circle_handle_t screen_device_create (unsigned width, unsigned height, unsigned display_num);

int device_write (circle_handle_t handle, const void *buffer, unsigned long size);

void timer_simple_ms_delay (unsigned msec);

#ifdef __cplusplus
}
#endif

#endif
