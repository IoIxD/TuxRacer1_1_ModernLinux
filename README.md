# Tux Racer 1.1 on Modern Linux

Revised installer and custom SDL 1.2 translation layer for getting the commercial/boxed version of Tux Racer's Linux port working on modern Linux. This normally doesn't run due to a vague error with `SDL_SetVideoMode` in SDL 1.2, and when I failed to be able to debug SDL enough to actually find out why it was thrown, I ended up just writing my own translation layer tailored to the game's needs, without SDL (which was funner anyways).

<img width="1919" height="931" alt="image" src="https://github.com/user-attachments/assets/e1b397f5-4534-453e-b00b-af891d401458" />

**Currently this only supports Wayland** due to my choice to forgo any libraries and write directly in it, and for that matter it won't have window decorations on GNOME because it relies on the XDG Decoration Manager protocol that GNOME doesn't want to support. It's recommended you use KDE. X11 support may be added in the future.

## Usage

Download the [zip](https://github.com/IoIxD/TuxRacer1_1_ModernLinux/releases) containing `setup.sh` and the `.so` file, and run the shell script. After it finishes installing, take note of any warnings/hints it gives you, including the instruction to move `libSDL-1.2.so.0` to the game's folder and to replace the game's copy of SDL mixer with a copy of SDL2 Mixer.

## Note for AMD GPUs

Mesa will default to using the Zink driver on AMD, which results in instability with the game's graphics and an eventual crash. To run this properly, you have to set `MESA_LOADER_DRIVER_OVERRIDE` to `llvmpipe`, either in your environment or by modifying lines 28 and 30 of the launcher script to be prefixed with `MESA_LOADER_DRIVER_OVERRIDE=llvmpipe`

## Checklist

- [x] Basic window
- [x] Keyboard/mouse support
- [ ] [Fullscreen](https://github.com/IoIxD/TuxRacer1_1_ModernLinux/issues/1/)
- [ ] X11 support
- [ ] Gamepad support
- [ ] HDR support?
- [ ] if somebody wants to add raytracing using this i think that'd be pretty funny
