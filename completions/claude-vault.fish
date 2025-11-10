# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_claude_vault_global_optspecs
	string join \n h/help V/version
end

function __fish_claude_vault_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_claude_vault_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_claude_vault_using_subcommand
	set -l cmd (__fish_claude_vault_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c claude-vault -n "__fish_claude_vault_needs_command" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -s V -l version -d 'Print version'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "add" -d 'Add a new profile'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "list" -d 'List all profiles'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "show" -d 'Show profile details'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "remove" -d 'Remove a profile'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "default" -d 'Set default profile'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "detect" -d 'Detect profile for current directory'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "init" -d 'Initialize project with a profile'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "exec" -d 'Execute command with profile credentials'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "env" -d 'Print environment variables for shell integration'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "completion" -d 'Generate shell completion scripts'
complete -c claude-vault -n "__fish_claude_vault_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand add" -s d -l description -d 'Profile description' -r
complete -c claude-vault -n "__fish_claude_vault_using_subcommand add" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand list" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand show" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand remove" -s y -l yes -d 'Skip confirmation prompt'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand remove" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand default" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand detect" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand init" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand exec" -s p -l profile -d 'Profile name (optional, uses detected/default profile)' -r
complete -c claude-vault -n "__fish_claude_vault_using_subcommand exec" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand env" -s p -l profile -d 'Profile name (optional, uses detected/default profile)' -r
complete -c claude-vault -n "__fish_claude_vault_using_subcommand env" -s h -l help -d 'Print help'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand completion" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "add" -d 'Add a new profile'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "list" -d 'List all profiles'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "show" -d 'Show profile details'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "remove" -d 'Remove a profile'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "default" -d 'Set default profile'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "detect" -d 'Detect profile for current directory'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "init" -d 'Initialize project with a profile'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "exec" -d 'Execute command with profile credentials'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "env" -d 'Print environment variables for shell integration'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "completion" -d 'Generate shell completion scripts'
complete -c claude-vault -n "__fish_claude_vault_using_subcommand help; and not __fish_seen_subcommand_from add list show remove default detect init exec env completion help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
