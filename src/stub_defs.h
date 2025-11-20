#include <GL/gl.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#define EXPORT __attribute__((visibility("default")))

typedef struct SDL_Joystick SDL_Joystick;

typedef enum SDLMod {
  KMOD_NONE = 0x0000,
  KMOD_LSHIFT = 0x0001,
  KMOD_RSHIFT = 0x0002,
  KMOD_LCTRL = 0x0040,
  KMOD_RCTRL = 0x0080,
  KMOD_LALT = 0x0100,
  KMOD_RALT = 0x0200,
  KMOD_LMETA = 0x0400,
  KMOD_RMETA = 0x0800,
  KMOD_NUM = 0x1000,
  KMOD_CAPS = 0x2000,
  KMOD_MODE = 0x4000,
  KMOD_RESERVED = 0x8000
} SDLMod;

typedef enum {
  /** @name ASCII mapped keysyms
   *  The keyboard syms have been cleverly chosen to map to ASCII
   */
  /*@{*/
  SDLK_UNKNOWN = 0,
  SDLK_FIRST = 0,
  SDLK_BACKSPACE = 8,
  SDLK_TAB = 9,
  SDLK_CLEAR = 12,
  SDLK_RETURN = 13,
  SDLK_PAUSE = 19,
  SDLK_ESCAPE = 27,
  SDLK_SPACE = 32,
  SDLK_EXCLAIM = 33,
  SDLK_QUOTEDBL = 34,
  SDLK_HASH = 35,
  SDLK_DOLLAR = 36,
  SDLK_AMPERSAND = 38,
  SDLK_QUOTE = 39,
  SDLK_LEFTPAREN = 40,
  SDLK_RIGHTPAREN = 41,
  SDLK_ASTERISK = 42,
  SDLK_PLUS = 43,
  SDLK_COMMA = 44,
  SDLK_MINUS = 45,
  SDLK_PERIOD = 46,
  SDLK_SLASH = 47,
  SDLK_0 = 48,
  SDLK_1 = 49,
  SDLK_2 = 50,
  SDLK_3 = 51,
  SDLK_4 = 52,
  SDLK_5 = 53,
  SDLK_6 = 54,
  SDLK_7 = 55,
  SDLK_8 = 56,
  SDLK_9 = 57,
  SDLK_COLON = 58,
  SDLK_SEMICOLON = 59,
  SDLK_LESS = 60,
  SDLK_EQUALS = 61,
  SDLK_GREATER = 62,
  SDLK_QUESTION = 63,
  SDLK_AT = 64,
  /*
     Skip uppercase letters
   */
  SDLK_LEFTBRACKET = 91,
  SDLK_BACKSLASH = 92,
  SDLK_RIGHTBRACKET = 93,
  SDLK_CARET = 94,
  SDLK_UNDERSCORE = 95,
  SDLK_BACKQUOTE = 96,
  SDLK_a = 97,
  SDLK_b = 98,
  SDLK_c = 99,
  SDLK_d = 100,
  SDLK_e = 101,
  SDLK_f = 102,
  SDLK_g = 103,
  SDLK_h = 104,
  SDLK_i = 105,
  SDLK_j = 106,
  SDLK_k = 107,
  SDLK_l = 108,
  SDLK_m = 109,
  SDLK_n = 110,
  SDLK_o = 111,
  SDLK_p = 112,
  SDLK_q = 113,
  SDLK_r = 114,
  SDLK_s = 115,
  SDLK_t = 116,
  SDLK_u = 117,
  SDLK_v = 118,
  SDLK_w = 119,
  SDLK_x = 120,
  SDLK_y = 121,
  SDLK_z = 122,
  SDLK_DELETE = 127,
  /* End of ASCII mapped keysyms */
  /*@}*/

  /** @name International keyboard syms */
  /*@{*/
  SDLK_WORLD_0 = 160, /* 0xA0 */
  SDLK_WORLD_1 = 161,
  SDLK_WORLD_2 = 162,
  SDLK_WORLD_3 = 163,
  SDLK_WORLD_4 = 164,
  SDLK_WORLD_5 = 165,
  SDLK_WORLD_6 = 166,
  SDLK_WORLD_7 = 167,
  SDLK_WORLD_8 = 168,
  SDLK_WORLD_9 = 169,
  SDLK_WORLD_10 = 170,
  SDLK_WORLD_11 = 171,
  SDLK_WORLD_12 = 172,
  SDLK_WORLD_13 = 173,
  SDLK_WORLD_14 = 174,
  SDLK_WORLD_15 = 175,
  SDLK_WORLD_16 = 176,
  SDLK_WORLD_17 = 177,
  SDLK_WORLD_18 = 178,
  SDLK_WORLD_19 = 179,
  SDLK_WORLD_20 = 180,
  SDLK_WORLD_21 = 181,
  SDLK_WORLD_22 = 182,
  SDLK_WORLD_23 = 183,
  SDLK_WORLD_24 = 184,
  SDLK_WORLD_25 = 185,
  SDLK_WORLD_26 = 186,
  SDLK_WORLD_27 = 187,
  SDLK_WORLD_28 = 188,
  SDLK_WORLD_29 = 189,
  SDLK_WORLD_30 = 190,
  SDLK_WORLD_31 = 191,
  SDLK_WORLD_32 = 192,
  SDLK_WORLD_33 = 193,
  SDLK_WORLD_34 = 194,
  SDLK_WORLD_35 = 195,
  SDLK_WORLD_36 = 196,
  SDLK_WORLD_37 = 197,
  SDLK_WORLD_38 = 198,
  SDLK_WORLD_39 = 199,
  SDLK_WORLD_40 = 200,
  SDLK_WORLD_41 = 201,
  SDLK_WORLD_42 = 202,
  SDLK_WORLD_43 = 203,
  SDLK_WORLD_44 = 204,
  SDLK_WORLD_45 = 205,
  SDLK_WORLD_46 = 206,
  SDLK_WORLD_47 = 207,
  SDLK_WORLD_48 = 208,
  SDLK_WORLD_49 = 209,
  SDLK_WORLD_50 = 210,
  SDLK_WORLD_51 = 211,
  SDLK_WORLD_52 = 212,
  SDLK_WORLD_53 = 213,
  SDLK_WORLD_54 = 214,
  SDLK_WORLD_55 = 215,
  SDLK_WORLD_56 = 216,
  SDLK_WORLD_57 = 217,
  SDLK_WORLD_58 = 218,
  SDLK_WORLD_59 = 219,
  SDLK_WORLD_60 = 220,
  SDLK_WORLD_61 = 221,
  SDLK_WORLD_62 = 222,
  SDLK_WORLD_63 = 223,
  SDLK_WORLD_64 = 224,
  SDLK_WORLD_65 = 225,
  SDLK_WORLD_66 = 226,
  SDLK_WORLD_67 = 227,
  SDLK_WORLD_68 = 228,
  SDLK_WORLD_69 = 229,
  SDLK_WORLD_70 = 230,
  SDLK_WORLD_71 = 231,
  SDLK_WORLD_72 = 232,
  SDLK_WORLD_73 = 233,
  SDLK_WORLD_74 = 234,
  SDLK_WORLD_75 = 235,
  SDLK_WORLD_76 = 236,
  SDLK_WORLD_77 = 237,
  SDLK_WORLD_78 = 238,
  SDLK_WORLD_79 = 239,
  SDLK_WORLD_80 = 240,
  SDLK_WORLD_81 = 241,
  SDLK_WORLD_82 = 242,
  SDLK_WORLD_83 = 243,
  SDLK_WORLD_84 = 244,
  SDLK_WORLD_85 = 245,
  SDLK_WORLD_86 = 246,
  SDLK_WORLD_87 = 247,
  SDLK_WORLD_88 = 248,
  SDLK_WORLD_89 = 249,
  SDLK_WORLD_90 = 250,
  SDLK_WORLD_91 = 251,
  SDLK_WORLD_92 = 252,
  SDLK_WORLD_93 = 253,
  SDLK_WORLD_94 = 254,
  SDLK_WORLD_95 = 255, /* 0xFF */
                       /*@}*/

  /** @name Numeric keypad */
  /*@{*/
  SDLK_KP0 = 256,
  SDLK_KP1 = 257,
  SDLK_KP2 = 258,
  SDLK_KP3 = 259,
  SDLK_KP4 = 260,
  SDLK_KP5 = 261,
  SDLK_KP6 = 262,
  SDLK_KP7 = 263,
  SDLK_KP8 = 264,
  SDLK_KP9 = 265,
  SDLK_KP_PERIOD = 266,
  SDLK_KP_DIVIDE = 267,
  SDLK_KP_MULTIPLY = 268,
  SDLK_KP_MINUS = 269,
  SDLK_KP_PLUS = 270,
  SDLK_KP_ENTER = 271,
  SDLK_KP_EQUALS = 272,
  /*@}*/

  /** @name Arrows + Home/End pad */
  /*@{*/
  SDLK_UP = 273,
  SDLK_DOWN = 274,
  SDLK_RIGHT = 275,
  SDLK_LEFT = 276,
  SDLK_INSERT = 277,
  SDLK_HOME = 278,
  SDLK_END = 279,
  SDLK_PAGEUP = 280,
  SDLK_PAGEDOWN = 281,
  /*@}*/

  /** @name Function keys */
  /*@{*/
  SDLK_F1 = 282,
  SDLK_F2 = 283,
  SDLK_F3 = 284,
  SDLK_F4 = 285,
  SDLK_F5 = 286,
  SDLK_F6 = 287,
  SDLK_F7 = 288,
  SDLK_F8 = 289,
  SDLK_F9 = 290,
  SDLK_F10 = 291,
  SDLK_F11 = 292,
  SDLK_F12 = 293,
  SDLK_F13 = 294,
  SDLK_F14 = 295,
  SDLK_F15 = 296,
  /*@}*/

  /** @name Key state modifier keys */
  /*@{*/
  SDLK_NUMLOCK = 300,
  SDLK_CAPSLOCK = 301,
  SDLK_SCROLLOCK = 302,
  SDLK_RSHIFT = 303,
  SDLK_LSHIFT = 304,
  SDLK_RCTRL = 305,
  SDLK_LCTRL = 306,
  SDLK_RALT = 307,
  SDLK_LALT = 308,
  SDLK_RMETA = 309,
  SDLK_LMETA = 310,
  SDLK_LSUPER = 311,  /**< Left "Windows" key */
  SDLK_RSUPER = 312,  /**< Right "Windows" key */
  SDLK_MODE = 313,    /**< "Alt Gr" key */
  SDLK_COMPOSE = 314, /**< Multi-key compose key */
                      /*@}*/

  /** @name Miscellaneous function keys */
  /*@{*/
  SDLK_HELP = 315,
  SDLK_PRINT = 316,
  SDLK_SYSREQ = 317,
  SDLK_BREAK = 318,
  SDLK_MENU = 319,
  SDLK_POWER = 320, /**< Power Macintosh power key */
  SDLK_EURO = 321,  /**< Some european keyboards */
  SDLK_UNDO = 322,  /**< Atari keyboard has Undo */
                    /*@}*/

  /* Add any other keys here */

  SDLK_LAST
} SDLKey;

// Todo: find out what's truly needed here.
typedef struct SDL_PixelFormat {
  void *palette;
  uint8_t BitsPerPixel;
  uint8_t BytesPerPixel;
  uint8_t Rloss, Gloss, Bloss, Aloss;
  uint8_t Rshift, Gshift, Bshift, Ashift;
  uint32_t Rmask, Gmask, Bmask, Amask;
  uint32_t colorkey;
  uint8_t alpha;
} SDL_PixelFormat;

typedef struct {
  uint32_t hw_available : 1;
  uint32_t wm_available : 1;
  uint32_t blit_hw : 1;
  uint32_t blit_hw_CC : 1;
  uint32_t blit_hw_A : 1;
  uint32_t blit_sw : 1;
  uint32_t blit_sw_CC : 1;
  uint32_t blit_sw_A : 1;
  uint32_t blit_fill;
  uint32_t video_mem;
  SDL_PixelFormat *vfmt;
} SDL_VideoInfo;

typedef struct SDL_Rect {
  int x, y;
  int w, h;
} SDL_Rect;

typedef enum {
  SDL_GL_RED_SIZE = 0,     //	Size of the framebuffer red component, in bits
  SDL_GL_GREEN_SIZE,       // Size of the framebuffer green component, in bits
  SDL_GL_BLUE_SIZE,        // Size of the framebuffer blue component, in bits
  SDL_GL_ALPHA_SIZE,       //	Size of the framebuffer alpha component, in bits
  SDL_GL_DOUBLEBUFFER,     //	0 or 1, enable or disable double buffering
  SDL_GL_BUFFER_SIZE,      //	Size of the framebuffer, in bits
  SDL_GL_DEPTH_SIZE,       //	Size of the depth buffer, in bits
  SDL_GL_STENCIL_SIZE,     //	Size of the stencil buffer, in bits
  SDL_GL_ACCUM_RED_SIZE,   //	Size of the accumulation buffer red component,
                           // in bits
  SDL_GL_ACCUM_GREEN_SIZE, //	Size of the accumulation buffer green component,
                           // in bits
  SDL_GL_ACCUM_BLUE_SIZE,  //	Size of the accumulation buffer blue component,
                           // in bits
  SDL_GL_ACCUM_ALPHA_SIZE, //	Size of the accumulation buffer alpha component,
                           // in bits
} SDL_GLattr;

typedef struct {
  uint8_t scancode;
  SDLKey sym;
  SDLMod mod;
  uint16_t unicode;
} SDL_keysym;

typedef struct {
  int version;
  int data;
} SDL_SysWMmsg;

typedef union {
  uint8_t type;
  struct {
    uint8_t type;
    uint8_t gain;
    uint8_t state;
  } active;
  struct {
    uint8_t type;
    uint8_t state;
    SDL_keysym keysym;
  } key;
  struct {
    uint8_t type;
    uint8_t state;
    uint16_t x, y;
    int16_t xrel, yrel;
  } motion;
  struct {
    uint8_t type;
    uint8_t button;
    uint8_t state;
    uint16_t x, y;
  } button;
  struct {
    uint8_t type;
    uint8_t which;
    uint8_t axis;
    int16_t value;
  } jaxis;
  struct {
    uint8_t type;
    uint8_t which;
    uint8_t ball;
    int16_t xrel, yrel;
  } jball;
  struct {
    uint8_t type;
    uint8_t which;
    uint8_t hat;
    uint8_t value;
  } jhat;
  struct {
    uint8_t type;
    uint8_t which;
    uint8_t button;
    uint8_t state;
  } jbutton;
  struct {
    uint8_t type;
    int w, h;
  } resize;
  struct {
    uint8_t type;
  } expose;
  struct {
    uint8_t type;
  } quit;
  struct {
    uint8_t type;
    int code;
    void *data1;
    void *data2;
  } user;
  struct {
    uint8_t type; /* Always SDL_SYSWMEVENT */
    SDL_SysWMmsg *msg;
  } syswm;
} SDL_Event;

typedef struct SDL_RWops {
  int (*seek)(struct SDL_RWops *context, int offset, int whence);
  int (*read)(struct SDL_RWops *context, void *ptr, int size, int maxnum);
  int (*write)(struct SDL_RWops *context, const void *ptr, int size, int num);
  int (*close)(struct SDL_RWops *context);

  uint32_t type;
  union {
#if defined(__WIN32__)
    struct {
      int append;
      void *h;
      struct {
        void *data;
        int size;
        int left;
      } buffer;
    } win32io;
#endif

    struct {
      int autoclose;
      FILE *fp;
    } stdio;

    struct {
      uint8_t *base;
      uint8_t *here;
      uint8_t *stop;
    } mem;

    struct {
      void *data1;
    } unknown;
  } hidden;
} SDL_RWops;

typedef struct SDL_Surface {
  uint32_t flags;          /* Read-only */
  SDL_PixelFormat *format; /* Read-only */
  int w, h;                /* Read-only */
  uint16_t pitch;          /* Read-only */
  void *pixels;            /* Read-write */

  /* clipping information */
  SDL_Rect clip_rect; /* Read-only */

  /* Reference count -- used when freeing surface */
  int refcount; /* Read-mostly */

  /* This structure also contains private fields not shown here */
} SDL_Surface;

EXPORT void SDL_Delay(uint32_t ms);
EXPORT int SDL_EnableKeyRepeat(int delay, int interval);
EXPORT char *SDL_GetError(void);
EXPORT uint8_t *SDL_GetKeyState(int *numkeys);
EXPORT SDLMod SDL_GetModState(void);
EXPORT uint8_t SDL_GetMouseState(int *x, int *y);
EXPORT SDL_VideoInfo *SDL_GetVideoInfo(void);
EXPORT int SDL_GL_GetAttribute(SDL_GLattr attr, int *value);
EXPORT void *SDL_GL_GetProcAddress(const char *proc);
EXPORT int SDL_GL_SetAttribute(SDL_GLattr attr, int value);
EXPORT void SDL_GL_SwapBuffers(void);
EXPORT int SDL_Init(uint32_t flags);
EXPORT int SDL_JoystickEventState(int state);
EXPORT int16_t SDL_JoystickGetAxis(SDL_Joystick *joystick, int axis);
EXPORT uint8_t SDL_JoystickGetButton(SDL_Joystick *joystick, int button);
EXPORT const char *SDL_JoystickName(int index);
EXPORT int SDL_JoystickNumAxes(SDL_Joystick *joystick);
EXPORT int SDL_JoystickNumButtons(SDL_Joystick *joystick);
EXPORT SDL_Joystick *SDL_JoystickOpen(int index);
EXPORT void SDL_LockAudio(void);
EXPORT int SDL_NumJoysticks(void);
EXPORT int SDL_PollEvent(SDL_Event *event);
EXPORT void SDL_Quit(void);
EXPORT SDL_RWops *SDL_RWFromFile(const char *file, const char *mode);
EXPORT SDL_Surface *SDL_SetVideoMode(int width, int height, int bpp,
                                     uint32_t flags);
EXPORT int SDL_ShowCursor(int toggle);
EXPORT void SDL_UnlockAudio(void);
EXPORT void SDL_WarpMouse(uint16_t x, uint16_t y);
EXPORT void SDL_WM_SetCaption(const char *title, const char *icon);
