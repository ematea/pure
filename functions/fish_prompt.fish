function fish_prompt
  printf "\n"

  if test ! -z "$SSH_TTY"
    set_color brblack
    printf "$USER"@(prompt_hostname)" "
  end

  set --local _pure_git_prompt (eval 'echo $_pure_git_prompt_'$fish_pid)
  set_color cyan
  printf "%s$_pure_git_prompt " (prompt_pwd)

  set_color yellow
  printf "$_pure_cmd_duration"

  if test -z "$_pure_exit_code" -o "$_pure_exit_code" = "0"
    set_color blue
  else
    set_color red
  end
  printf "\n❯ "
  set_color normal
end
