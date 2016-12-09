#!/usr/bin/env powershell

$direction_string = Get-Content day1.input
$direction_string
$directions = $direction_string.Split(",").Trim()

foreach ($line in $directions) {
	"'" + $line + "'"
}

