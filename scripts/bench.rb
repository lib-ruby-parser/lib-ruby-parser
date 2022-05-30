#!/usr/bin/env ruby

# This script is driven by scripts/bench.sh

require 'ripper'

files = Dir['gems/repos/**/*.rb'].map { |f| [f, File.read(f)] }

def measure(files_count)
    start = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    yield
    now = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    puts "Time taken: #{now - start} (total files: #{files_count})"
end

GC.disable

measure(files.count) do
    files.each do |(filepath, src)|
        Ripper.sexp(src, filepath)
    end
end
