#include "wrapper.h"
#include <circle/devicenameservice.h>
#include <circle/actled.h>
#include <circle/screen.h>
#include <circle/timer.h>
#include <assert.h>

static CDeviceNameService *s_pDeviceNameService = nullptr;

void circle_init ()
{
	assert (!s_pDeviceNameService);
	s_pDeviceNameService = new CDeviceNameService;
	assert (s_pDeviceNameService);
}

circle_handle_t act_led_create ()
{
	return new CActLED (false);
}

void act_led_destroy (circle_handle_t handle)
{
	delete static_cast<CActLED *> (handle);
}

void act_led_on (circle_handle_t handle)
{
	static_cast<CActLED *> (handle)->On ();
}

void act_led_off (circle_handle_t handle)
{
	static_cast<CActLED *> (handle)->Off ();
}

void act_led_blink (circle_handle_t handle, unsigned count, unsigned msec_on, unsigned msec_off)
{
	static_cast<CActLED *> (handle)->Blink (count, msec_on, msec_off);
}

circle_handle_t screen_device_create (unsigned width, unsigned height, unsigned display_num)
{
	CScreenDevice *screen = new CScreenDevice (width, height, false, display_num);

	if (!screen->Initialize ())
	{
		return 0;
	}

	return screen;
}

int device_write (circle_handle_t handle, const void *buffer, unsigned long size)
{
	return static_cast<CDevice *> (handle)->Write (buffer, size);
}

void timer_simple_ms_delay (unsigned msec)
{
	CTimer::SimpleMsDelay (msec);
}
