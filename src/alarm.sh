#!/data/data/com.termux/files/usr/bin/bash

Help()
{
    echo "\
Usage: termux-alarm <hour> <minute> [-m message] [-h]

Android alarm wrapper script

-m message    alarm message
-h            display this help message and exit"
}

while getopts ":m:h" option; do
  case $option in
      h)
          Help
          exit
          ;;
      m)
	  message="$OPTARG"
          ;;
      \?) # Invalid option
          echo "Invalid option -$OPTARG"
          Help
          exit
          ;;
  esac
done

shift $((OPTIND -1))

cmd="am start -a android.intent.action.SET_ALARM --ei android.intent.extra.alarm.HOUR ${1} --ei android.intent.extra.alarm.MINUTES ${2} --ez android.intent.extra.alarm.SKIP_UI true"

if [ -n "$message" ]; then
    cmd="${cmd} --es android.intent.extra.alarm.MESSAGE \"$message\""
fi

eval $cmd
