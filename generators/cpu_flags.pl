#!/usr/bin/env perl

use v5.36;

my @cpu_flags_out = qx(qemu-system-x86_64 -cpu help);

my @all_flags;
my $found = 0;
foreach my $line (@cpu_flags_out) {
    if ($found) {
        $line =~ s/\s*//;
        chomp $line;
        my @line_flags = split /\s+/, $line;
        foreach my $flag (@line_flags) {
            push @all_flags, $flag;
        }
    }
    if ($line =~ /^Recognized/) {
        $found = 1;
    }
}

# remove duplicates, why are these there?
my %seen;
my @flags;

foreach my $flag (@all_flags) {
    next if (exists $seen{$flag});
    $seen{$flag} = 1;
    push @flags, $flag;
}
say 'use crate::to_command::{ToCommand, ToArg};';

say "pub enum CPUFlags {";

foreach my $flag (@flags) {
    say "    /// $flag ";
    say "    " . ucfirst(clean($flag)) . ",";
}

say "}";

say << "EOF";
impl ToCommand for CPUFlags {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
EOF

foreach my $flag (@flags) {
    say "            CPUFlags::" . ucfirst(clean($flag)) . " => cmd.push(\"$flag\".to_string()),";
}

say << "EOF";

        }
        cmd
    }
}
EOF

say << "EOF";
impl ToArg for CPUFlags {
    fn to_arg(&self) -> &str {
        match self {
EOF

foreach my $flag (@flags) {
    say "            CPUFlags::" . ucfirst(clean($flag)) . ' => "' . $flag . '",';
}

say << "EOF";
        }
    }
}
EOF

sub clean {
    my ($s) = @_;

    $s =~ s/^(\d)/X$1/;
    $s =~ s/-v/V/;
    $s =~ s/-//g;
    $s =~ s/_//g;
    $s =~ s/\./_/g;

    $s;
}
