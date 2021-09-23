def abort(message)
    $stderr.puts(message)
    exit 1
end

if ARGV.size < 2
    abort 'Usage: ruby assert_defs.rb <path/to/header.{h,hpp}>.. <path/to/impl.{c,cpp}>..'
end
header, *impls = ARGV

RE = /lib_ruby_parser__external__[a-z_]+/

module CC
    def self.fns(filepath)
        cc = ENV.fetch('CC') { abort 'CC env variable must be specified for .c/.h files' }
        expanded = `cat #{filepath} | grep -v "#include" | #{cc} -x c -E -`
        expanded.scan(RE).uniq.sort
    end
end

module CXX
    def self.fns(filepath)
        cxx = ENV.fetch('CXX') { abort 'CXX env variable must be specified for .cpp/.hpp files' }
        expanded = `cat #{filepath} | grep -v "#include" | #{cxx} -E -`
        expanded.scan(RE).uniq.sort
    end
end

module Compiler
    def self.for(filepath)
        ext = File.extname(filepath)
        case ext
        when '.c', '.h' then CC
        when '.cpp', '.hpp' then CXX
        else
            abort "Unknown file extension #{ext}, supported are: .h, .hpp, .c, .cpp"
        end
    end
end

COMPILER = Compiler.for(header)
impls.each do |impl|
    if Compiler.for(impl) != COMPILER
        abort "Header is #{COMPILER}, impl is #{Compiler.for(impl)}, aborting"
    end
end

puts <<~HERE
    Running with:
    header   = #{header}
    impls    = #{impls.join(", ")}
    COMPILER = #{COMPILER}
HERE

header_fns = COMPILER.fns(header)
impls = impls.map { |impl| [impl, COMPILER.fns(impl)] }.to_h

puts '== Searching for declarations that have no implementation...'
header_fns.each do |fn|
    found = false
    impls.each do |impl, fns|
        if fns.include?(fn)
            if found
                abort "#{fn} is defined multiple times"
            end
            found = true
            puts "#{fn} - found in #{impl}"
        end
    end
    abort "Couldn't find implementation of #{fn}" unless found
end

puts "\n\n== Searching for implementations that are not declared..."
impls.each do |impl, fns|
    fns.each do |fn|
        if header_fns.include?(fn)
            puts "#{fn} - found"
        else
            abort "Coulnd't find definition of #{fn}"
        end
    end
end
