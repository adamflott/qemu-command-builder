#!/usr/bin/env perl

use v5.36;

my %cpus_by_arch;

# example output:
# > qemu-system-x86_64 -cpu help
# Available CPUs:
#   486                   (alias configured by machine type)
#   486-v1
#   Broadwell             (alias configured by machine type)
#   Broadwell-IBRS        (alias of Broadwell-v3)
# Recognized CPUID flags:
#   3dnow 3dnowext 3dnowprefetch abm ace2 ace2-en acpi adx aes amd-no-ssb

foreach my $arch (qw(aarch64 x86_64)) {
    my $cmd = "qemu-system-$arch";
    foreach my $line (qx($cmd -cpu help)) {
        last if ($line =~ /^Recognized CPUID flags:/);
        next unless $line =~ /^  /;
        chomp $line;
        $line =~ s/^\s+//;
        my @cpu_fields = split /\s+/, $line;
        my $cpu_name   = shift @cpu_fields;

        push @{$cpus_by_arch{$arch}}, {name => $cpu_name, desc => join ' ', @cpu_fields};
    }

    push @{$cpus_by_arch{$arch}},
      {
        name => 'Host',
        desc => 'Enables all features supported by the accelerator in the current host'
      };
}

say 'use crate::to_command::{ToCommand, ToArg};';

foreach my $arch (keys %cpus_by_arch) {
    my $cpu_type_ns = "CpuType" . ucfirst $arch;
    say "pub enum $cpu_type_ns {";

    foreach my $cpu (@{$cpus_by_arch{$arch}}) {
        my $cpu_name = clean($cpu->{name});

        $cpu_name =~ s/^\s+//;

        if ($cpu->{desc}) {
            say "    /// $cpu->{desc}";
        }
        else {
            say "    /// $cpu->{name}";
        }
        say "    " . ucfirst $cpu_name . ",";
    }

    say "}";

    say << "EOF";
impl ToCommand for $cpu_type_ns {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
EOF

    foreach my $cpu (@{$cpus_by_arch{$arch}}) {
        say "            ${cpu_type_ns}::"
          . ucfirst(clean($cpu->{name}))
          . ' => cmd.push("'
          . lc($cpu->{name})
          . '".to_string()),';
    }

    say << "EOF";
        }
        cmd
    }
}
EOF

    say << "EOF";
impl ToArg for $cpu_type_ns {
    fn to_arg(&self) -> &str {
        match self {
EOF

    foreach my $cpu (@{$cpus_by_arch{$arch}}) {
        say "            ${cpu_type_ns}::" . ucfirst(clean($cpu->{name})) . ' => "' . $cpu->{name} . '",';
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

    $s;
}
