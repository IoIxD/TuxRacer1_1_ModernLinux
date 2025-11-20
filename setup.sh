#!/bin/sh
#
# Product setup script - Loki Entertainment Software
script=$0

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
arch="x86"
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
    setup_bin="setup.data/bin/$os/$arch/$libc/$setup"
    if [ ! -f "$setup_bin" ]; then
        setup_bin="setup.data/bin/$os/$arch/$setup"
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
        echo "The setup program seems to have failed on $arch/$libc"
        status=1
    }
fi
exit $status
