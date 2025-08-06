#!/usr/bin/env perl

use v5.36;

my %machines_by_arch;

# example output:
# > qemu-system-x86_64 -machine help
# Supported machines are:
# microvm              microvm (i386)
# pc-i440fx-9.2        Standard PC (i440FX + PIIX, 1996)
# pc-i440fx-9.1        Standard PC (i440FX + PIIX, 1996)

foreach my $arch (qw(aarch64 x86_64)) {
    my $cmd = "qemu-system-$arch";

    foreach my $line (qx($cmd -machine help)) {
        next if $line =~ /^Supported machines are:/;
        chomp $line;

        my @machine_fields = split /\s+/, $line;
        my $machine_name   = shift @machine_fields;
        push @{$machines_by_arch{$arch}}, {name => $machine_name, desc => join ' ', @machine_fields};
    }
}

say 'use crate::to_command::{ToCommand, ToArg};';

foreach my $arch (keys %machines_by_arch) {
    my $machine_type_ns = "Machine" . ucfirst $arch;
    say "pub enum $machine_type_ns {";

    foreach my $machine (@{$machines_by_arch{$arch}}) {
        my $machine_str = clean($machine->{name});
        say "    /// " . $machine->{desc};
        say "    " . ucfirst($machine_str) . ",";
    }

    say "}";

    say << "EOF";
impl ToCommand for $machine_type_ns {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
EOF

    foreach my $machine (@{$machines_by_arch{$arch}}) {
        my $machine_orig_name = $machine->{name};
        my $machine_str       = clean($machine->{name});
        say "            ${machine_type_ns}::"
          . ucfirst $machine_str
          . " => { cmd.push(\"${machine_orig_name}\".to_string()); } ";
    }

    say << "EOF";

        }
        cmd
    }
}
EOF

    say << "EOF";
impl ToArg for $machine_type_ns {
    fn to_arg(&self) -> &str {
        match self {
EOF

    foreach my $machine (@{$machines_by_arch{$arch}}) {
        say "            ${machine_type_ns}::" . ucfirst(clean($machine->{name})) . ' => "' . $machine->{name} . '",';
    }

    say << "EOF";
        }
    }
}
EOF
}

sub clean {
    my ($s) = @_;

    $s =~ s/^(\d)/X$1/;
    $s =~ s/-v/V/;
    $s =~ s/-//g;
    $s =~ s/_//g;
    $s =~ s/\./_/g;

    $s;
}
