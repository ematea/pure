function fish_prompt
  printf "\n\033[2K"

  if test ! -z "$SSH_TTY"
    set_color $pure_color_ssh
    printf "$USER"@(prompt_hostname)" "
  end

  set --local _pure_git_prompt (eval 'echo $_pure_git_prompt_'$fish_pid)
  set_color $pure_color_pwd
  printf "%s" (prompt_pwd)

  set_color normal
  printf "$_pure_git_prompt "

  set_color $pure_color_duration
  printf "$_pure_duration"

  if test -z "$_pure_exit_code" -o "$_pure_exit_code" = "0"
    set_color $pure_color_prompt
  else
    set_color $pure_color_prompt_failed
  end
  printf "\n$pure_symbol_prompt "
  set_color normal
end
