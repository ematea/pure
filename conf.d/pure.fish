function _pure_postexec --on-event fish_postexec
  set --global _pure_exit_code $status
  test "$CMD_DURATION" -lt 1000 && set _pure_duration && return

  set --local secs (math --scale=1 $CMD_DURATION/1000 % 60)
  set --local mins (math --scale=0 $CMD_DURATION/60000 % 60)
  set --local hours (math --scale=0 $CMD_DURATION/3600000)

  test $hours -gt 0 && set --local --append out $hours"h"
  test $mins -gt 0 && set --local --append out $mins"m"
  test $secs -gt 0 && set --local --append out $secs"s"

  set --global _pure_duration "$out"
end

function _pure_prompt --on-event fish_prompt
  fish --private --command "
    if test \"\$pure_remove_git_parenthesis\" = \"true\"
      set --universal _pure_git_prompt_$fish_pid (fish_git_prompt ' ')
    else
      set --universal _pure_git_prompt_$fish_pid (fish_git_prompt)
    end
    kill -WINCH $fish_pid
  " &
end

function _pure_fish_exit --on-event fish_exit
  set -e _pure_git_prompt_$fish_pid
end

set --query pure_color_ssh || set --global pure_color_ssh normal
set --query pure_color_pwd || set --global pure_color_pwd normal
set --query pure_color_duration || set --global pure_color_duration normal
set --query pure_symbol_prompt || set --global pure_symbol_prompt ❯
set --query pure_color_prompt || set --global pure_color_prompt normal
set --query pure_color_prompt_failed || set --global pure_color_prompt_failed red
set --query pure_remove_git_parenthesis || set --global pure_remove_git_parenthesis no
