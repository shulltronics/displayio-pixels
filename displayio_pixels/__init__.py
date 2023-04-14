# A class to use the Rust Pixels as a DisplayIO "display"
# By Shulltronics, 2023

import sys
import time
from PIL import Image
import displayio

from .displayio_pixels import Orientation

from dataclasses import astuple

_INIT_SEQUENCE = None

class PixelsDisplay(displayio.Display):

    def __init__(self, width, height, **kwargs):
        self.running = True
        # construct the Rust object
        self.pixels = displayio_pixels.PixelsDisplay(width, height)
        #self.pixels.set_orientation(Orientation.LANDSCAPE)
        (self._width, self._height) = self.pixels.get_size()
        # initialize the super class, displayio.Display.
        super().__init__(None, _INIT_SEQUENCE, width=self._width, height=self._height, **kwargs)

    def _initialize(self, init_sequence):
        """ We have to override this method because we're not using an init sequence """
        pass

    def refresh(self, *, target_frames_per_second=30, minimum_frames_per_second=1):
        """
        When auto refresh is off, waits for the target frame rate and then refreshes the
        display, returning True. If the call has taken too long since the last refresh call
        for the given target frame rate, then the refresh returns False immediately without
        updating the screen to hopefully help getting caught up.
        If the time since the last successful refresh is below the minimum frame rate, then
        an exception will be raised. Set minimum_frames_per_second to 0 to disable.
        When auto refresh is on, updates the display immediately. (The display will also
        update without calls to this.)
        """
        if self.running:
            self._subrectangles = []

            # Go through groups and and add each to buffer
            if self._core._current_group is not None:
                buffer = Image.new("RGBA", (self._core._width, self._core._height))
                print(type(buffer))
                # Recursively have everything draw to the image
                # pylint: disable=protected-access
                self._core._current_group._fill_area(buffer)
                # pylint: disable=protected-access
                # save image to buffer (or probably refresh buffer so we can compare)
                self._buffer.paste(buffer)

            self._subrectangles = self._core.get_refresh_areas()

            for area in self._subrectangles:
                self._refresh_display_area(area)


    def _refresh_display_area(self, rectangle):
        """Loop through dirty rectangles and redraw that area."""
        # extract the dirty rectangle and convert it to RGBA format
        img = self._buffer.convert("RGBA").crop(astuple(rectangle))
        raw_str = img.tobytes("raw", "RGBA")
        self.pixels.write_bytes(raw_str)

    def get_mouse_clicks(self):
        event_return = None
        # TODO --> figure how to get keyboard / touchscreen / gamepad inputs 
        # without a display server
        return event_return

    def set_orientation(self, o = 0):
        if o == 90:
            self.pixels.set_orientation(Orientation.LANDSCAPE)
            (self._width, self._height) = (self._height, self._width)
            # self._rotation = 90
        else:
            self.pixels.set_orientation(Orientation.PORTRAIT)
            self._rotation = 0

    def quit(self):
        """
        Close the program!
        """
        print("Closing the program... goodbye!")
        self.running = False
