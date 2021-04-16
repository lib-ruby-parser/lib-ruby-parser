VERSION_LINE_RE = /\Aversion = "(\d)\.(\d)\.(\d)"\z/

cargo_toml = File.read('Cargo.toml')
major, minor, patch =
    cargo_toml
    .split("\n")
    .map { |line| line.match(VERSION_LINE_RE) }
    .compact
    .first
    .captures
    .map(&:to_i)

Version = Struct.new(:major, :minor, :patch) do
    def next_version
        major = self.major
        minor = self.minor
        patch = self.patch

        case ARGV[0]
        when 'major'
            major += 1
            minor = patch = 0
        when 'minor'
            minor += 1
            patch = 0
        when 'patch'
            patch += 1
        else
            puts "Unknown ARGV[0] #{ARGV[0].inspect}, expected: major/minor/patch"
            exit 1
        end

        Version.new(major, minor, patch)
    end

    def to_s
        "#{major}.#{minor}.#{patch}"
    end

    def rewrite_in_text(text)
        current_version = self
        next_version = self.next_version

        {
            current_version.to_s => next_version.to_s,
        }.each do |pattern, replacement|
            text = text.gsub(pattern, replacement)
        end

        text
    end

    def rewrite_in_file(path)
        File.write(path, rewrite_in_text(File.read(path)))
    end
end

version = Version.new(major, minor, patch)

puts "current version = #{version.to_s}"
puts "next version = #{version.next_version.to_s}"

FILES = [
    'Cargo.toml'
]

FILES.each do |file|
    puts "[+] Updating #{file}"
    version.rewrite_in_file(file)
end

`git add #{FILES.join(' ')}`
`git commit -m "bump v#{version.next_version.to_s}"`
`git tag v#{version.next_version.to_s}`

puts <<~TEXT
Done!

git push && git push --tags

Or to undo

git tag -d v#{version.next_version.to_s} && git reset --hard head~1
TEXT
