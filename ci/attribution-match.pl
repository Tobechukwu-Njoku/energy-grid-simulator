#!/usr/bin/env perl
# Reports attribution-policy matches (D-52, ADR 0001). CI REJECTS matches; it never
# rewrites. Loads patterns exactly as .githooks/commit-msg does: strict UTF-8,
# blank/comment lines ignored, CRLF accepted, case-insensitive, and an invalid,
# empty or empty-string-matching rule set is a hard failure.
#
# Usage: attribution-match.pl PATTERNS < items.json
# items.json is an array of {"id": str, "mode": "message"|"text", "text_hex": str}.
# "message" mode mirrors the hook: lines after a Git scissors line are ignored.
# Text is given as hex of the raw bytes so invalid UTF-8 is detected here, where
# the hook would also refuse it. Prints one JSON object per finding:
#   {"id": ..., "line": n, "text": ..., "reason": ...}
# Exit status: 0 = scan completed (findings may exist), 2 = configuration error.
use strict;
use warnings;
use Encode qw(decode FB_CROAK);
use JSON::PP;

sub fail { print STDERR "attribution-match: $_[0]\n"; exit 2 }

sub strict_utf8 {
    my ($bytes) = @_;
    my $copy = $bytes;
    my $decoded = eval { decode('UTF-8', $copy, FB_CROAK) };
    return $@ ? undef : $decoded;
}

my ($pattern_path) = @ARGV;
fail('usage: attribution-match.pl PATTERNS < items.json') unless defined $pattern_path;
open my $pattern_file, '<:raw', $pattern_path or fail("$pattern_path: $!");
my $pattern_bytes = do { local $/; <$pattern_file> };
close $pattern_file;
my $pattern_text = strict_utf8($pattern_bytes);
fail("$pattern_path: invalid UTF-8") unless defined $pattern_text;

my @patterns;
for my $pattern (split /\n/, $pattern_text) {
    $pattern =~ s/\r$//;
    next if $pattern =~ /^\s*(?:#|$)/;
    my $compiled = eval { qr/$pattern/i };
    fail("invalid pattern: $@") unless $compiled;
    fail('unsafe empty-string attribution pattern') if '' =~ $compiled;
    push @patterns, $compiled;
}
fail('no attribution patterns loaded') unless @patterns;

my $items = eval { decode_json(do { local $/; <STDIN> }) };
fail("bad input JSON: $@") unless ref $items eq 'ARRAY';

my $json = JSON::PP->new->canonical->ascii;
for my $item (@$items) {
    my $text = strict_utf8(pack 'H*', $item->{text_hex});
    unless (defined $text) {
        print $json->encode({ id => $item->{id}, line => 0, text => '', reason => 'invalid UTF-8' }), "\n";
        next;
    }
    my $number = 0;
    for my $line (split /\n/, $text) {
        $number++;
        last if $item->{mode} eq 'message' && $line =~ /^# -+ >8 -+/;
        (my $bare = $line) =~ s/\r$//;
        if (grep { $bare =~ $_ } @patterns) {
            print $json->encode({ id => $item->{id}, line => $number, text => $bare, reason => 'attribution' }), "\n";
        }
    }
}
exit 0;
