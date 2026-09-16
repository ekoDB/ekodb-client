#!/usr/bin/env perl
use strict;
use warnings;
use POSIX qw(setpgid);

my $seconds = shift @ARGV;
die "usage: $0 <positive-seconds> <command> [args...]\n"
    unless defined $seconds && $seconds =~ /^[1-9][0-9]*$/ && @ARGV;

my $pid = fork();
die "failed to fork: $!\n" unless defined $pid;

if ($pid == 0) {
    # Example runs are automated and must never read from the invoking terminal.
    # The child uses its own process group so it and its descendants can be
    # terminated together; inheriting a terminal here can suspend that group
    # with SIGTTIN when a launcher such as Gradle probes stdin during shutdown.
    open STDIN, '<', '/dev/null' or die "failed to detach child stdin: $!\n";
    setpgid(0, 0) == 0 or die "failed to create child process group: $!\n";
    exec @ARGV or die "failed to execute $ARGV[0]: $!\n";
}

# Close the race where the parent can set the group before the child does.
setpgid($pid, $pid);

sub stop_child_group {
    my ($signal) = @_;
    kill $signal, -$pid;
}

my %signal_number = (HUP => 1, INT => 2, TERM => 15);
for my $signal (keys %signal_number) {
    $SIG{$signal} = sub {
        alarm 0;
        stop_child_group($signal);
        select undef, undef, undef, 1.0;
        stop_child_group('KILL');
        waitpid($pid, 0);
        exit 128 + $signal_number{$signal};
    };
}

$SIG{ALRM} = sub {
    stop_child_group('TERM');
    select undef, undef, undef, 1.0;
    stop_child_group('KILL');
    waitpid($pid, 0);
    print STDERR "example timed out after ${seconds}s\n";
    exit 124;
};

alarm $seconds;
waitpid($pid, 0);
alarm 0;

my $status = $?;
exit 127 if $status == -1;
exit 128 + ($status & 127) if $status & 127;
exit $status >> 8;
