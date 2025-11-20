#!/bin/sh
#
# Product setup script - Loki Entertainment Software
script=$0

deps="libGLU.so.1 libGL.so.1 libSDL_mixer-1.2.so.0 libSDL-1.2.so.0 libpthread.so.0 libtcl.so.0 libpng.so.2 libSM.so.6 libICE.so.6 libX11.so.6 libXi.so.6 libXext.so.6 libXmu.so.6 libXt.so.6 libdl.so.2 libm.so.6 libc.so.6"

count=0
while [ -L "$script" ] ; do
    script=`perl -e "print readlink(\"$script\"), \"\n\""`
    count=`expr $count + 1`

    if [ $count -gt 100 ] ; then
       echo "Too many symbolic links"
       exit 1
    fi
done

dir=$1
if [ "$dir" == "" ]; then
    echo "Directory to mounted CD not provided."
    exit 1
fi

cd $dir/"program files/Sunspire Studios/Tux Racer"
pwd

# Return the appropriate version string
DetectLIBC()
{
      status=1
	  if [ `uname -s` != Linux ]; then
		  echo "glibc-2.1"
		  return $status
	  fi
      if [ -f `echo /lib/libc.so.6* | tail -1` ]; then
	      if grep -F GLIBC_2.1 /lib/libc.so.6* 2>&1 >/dev/null; then
	              echo "glibc-2.1"
	      else
	              echo "glibc-2.0"
	      fi
      elif [ -f /lib/libc.so.5 ]; then
	      echo "libc5"
	      status=0
      else
	      echo "unknown"
      fi
      return $status
}

# Detect the Linux environment
libc=`DetectLIBC`
os=`uname -s`

# Find the installation program
try_run()
{
    setup=$1
    shift
    fatal=$1
    if [ "$1" != "" ]; then
        shift
    fi

    # First find the binary we want to run
    failed=0
    setup_bin="setup.data/bin/$os/x86/$libc/$setup"
    if [ ! -f "$setup_bin" ]; then
        setup_bin="setup.data/bin/$os/x86/$setup"
    fi
    if [ "$failed" -eq 1 ]; then
        if [ "$fatal" != "" ]; then
            cat <<__EOF__
__EOF__
        fi
        return $failed
    fi

    # Try to run the binary
    # The executable is here but we can't execute it from CD
    setup="$HOME/.setup$$"
    cp "$setup_bin" "$setup"
    chmod 700 "$setup"
    if [ "$fatal" != "" ]; then
        "$setup" $*
        failed=$?
        if [ $failed == 127 ]; then
            echo "Hint: if the error is file not found, this indicates an inability to run 32-bit programs. Check how to run them on your distro."
        fi
    else
        "$setup" $* 2>/dev/null
        failed=$?
    fi
    rm -f "$setup"
    return $failed
}

sdl1_2_mixer_found() {
    echo ""
    echo "URGENT: SDL1.2 Mixer found in $1."
    echo "This is a bad thing, actually, as SDL1.2 Mixer does not work with our translation layer."
    echo "Since SDL2 Mixer is API compatible, you should copy that into the game's directory under the name it expects"
    echo ""
}

# Try to run the setup program
status=0
rm -f "$setup"
try_run setup.gtk $*
status=$?
if [ $status -ne 0 ]; then
    if [ $status -eq 127 ]; then
        valid=0
        while [ $valid -eq 0 ]; do
            read -p "You may run the CLI installer, but you may not be able to run the installed game (y/n):" yn
            case $yn in
                [Yy]* ) valid=1; break;;
                [Nn]* ) valid=1; exit;;
            esac
        done
    fi
    try_run setup $* fatal || {
        echo "The setup program seems to have failed on  x86/$libc"
        status=1
    }
else
    echo ""
    echo ""
    echo ""
    any32bitgone=0
    mixererr=0
    for dep in $deps; do
        is_mixer=0
        if [ "$dep" == "libSDL_mixer-1.2.so.0" ]; then
            is_mixer=1
        fi
        if [ -f /usr/lib32/$dep ]; then
            if [ $is_mixer -eq 1 ]; then
                sdl1_2_mixer_found "/usr/lib32"
            fi
        else
            any32bitgone=1
            echo -n "WARNING: $dep not found in /usr/lib32/. "
            if [ $is_mixer -eq 1 ]; then
                echo -n "Note that for SDL_mixer, you actually should install SDL2_Mixer (which is API compatible) and copy it into the game's directory."
                mixererr=1
            fi
            echo ""
        fi
    done
    if [ $any32bitgone -eq 1 ]; then
        echo "The game may fail to launch if there's missing dependencies, though some of these may also be present on the disc and work fine."
        echo "If not, make sure you installed them, and make sure you have the 32-bit version."
        echo ""
    fi
    echo "Next instructions:"
    echo "- Replace the game's libSDL-1.2.so.0 with our own (target/i686-unknown-linux-gnu/___/libSDL_1_2.so if you compiled yourself)"
    if [ $mixererr -eq 1 ]; then
        echo "- Replace the game's SDL1.2-mixer with a copy of SDL2 mixer (which is API compatible but actually works on modern systems)"
    fi
    if [ $status -eq 127 ]; then
        echo "- Remember to look into how you can execute i686 binaries on your system."
    fi
    echo ""
    echo ""
    echo ""
fi
exit $status
