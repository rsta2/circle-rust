#include "wrapper.h"
#include <circle/koptions.h>
#include <circle/devicenameservice.h>
#include <circle/actled.h>
#include <circle/screen.h>
#include <circle/exceptionhandler.h>
#include <circle/timer.h>
#include <circle/logger.h>
#include <assert.h>

static CKernelOptions *s_pKernelOptions = nullptr;
static CDeviceNameService *s_pDeviceNameService = nullptr;
static CExceptionHandler *s_pExceptionHandler = nullptr;

void circle_init (void)
{
	assert (!s_pKernelOptions);
	s_pKernelOptions = new CKernelOptions;
	assert (s_pKernelOptions);

	assert (!s_pDeviceNameService);
	s_pDeviceNameService = new CDeviceNameService;
	assert (s_pDeviceNameService);

	assert (!s_pExceptionHandler);
	s_pExceptionHandler = new CExceptionHandler;
	assert (s_pExceptionHandler);
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

unsigned kernel_options_get_width (void)
{
	assert (s_pKernelOptions);
	return s_pKernelOptions->GetWidth ();
}

unsigned kernel_options_get_height (void)
{
	assert (s_pKernelOptions);
	return s_pKernelOptions->GetHeight ();
}

unsigned kernel_options_get_log_level (void)
{
	assert (s_pKernelOptions);
	return s_pKernelOptions->GetLogLevel ();
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

void logger_create (circle_handle_t target_handle, unsigned log_level)
{
	CLogger *pLogger = new CLogger (log_level);
	assert (pLogger);

	pLogger->Initialize (static_cast<CDevice *> (target_handle));
}

void logger_write (const char *source, enum log_severity_t severity, const char *msg)
{
	TLogSeverity Severity = LogDebug;
	switch (severity)
	{
	case log_panic:		Severity = LogPanic;	break;
	case log_error:		Severity = LogError;	break;
	case log_warning:	Severity = LogWarning;	break;
	case log_notice:	Severity = LogNotice;	break;
	case log_debug:		Severity = LogDebug;	break;
	}

	CLogger::Get ()->Write (source, Severity, msg);
}
