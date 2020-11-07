ROOT = File.expand_path('', __dir__)
REPOS_ROOT = File.join(ROOT, 'repos')
require 'fileutils'
FileUtils.mkdir_p(REPOS_ROOT)

require 'open-uri'
require 'json'

def get_json(url)
    JSON.parse(URI.open(url).read)
end

def each_gem_name
    return to_enum(__method__) unless block_given?

    # Top 50 gems from rubygems.org
    get_json('https://rubygems.org/api/v1/downloads/all.json')['gems'].each do |(gem_data, _downloads)|
        full_name = gem_data['full_name']
        version = gem_data['number']
        name = full_name.delete_suffix("-#{version}")
        yield name
    end

    # Rails parts
    yield 'rails'
    yield 'activesupport'
    yield 'actionpack'
    yield 'actionview'
    yield 'activemodel'
    yield 'activerecord'
    yield 'actionmailer'
    yield 'activejob'
    yield 'actioncable'
    yield 'activestorage'
    yield 'actionmailbox'
    yield 'actiontext'
    yield 'railties'
end

def download_gem(gem_name)
    gem_info = get_json("https://rubygems.org/api/v1/gems/#{gem_name}.json")
    gem_uri = gem_info['gem_uri']

    if File.exists?("#{REPOS_ROOT}/#{gem_name}.gem")
        puts "Skipping wget #{REPOS_ROOT}/#{gem_name}.gem"
    else
        `wget #{gem_uri} -O #{REPOS_ROOT}/#{gem_name}.gem`
    end
end

def unpack_gem(gem_name)
    if File.directory?("#{REPOS_ROOT}/#{gem_name}")
        puts "Skipping gem unpack #{REPOS_ROOT}/#{gem_name}.gem"
    else
        `gem unpack #{REPOS_ROOT}/#{gem_name}.gem --target #{REPOS_ROOT}`
    end
end

each_gem_name.sort.uniq.each do |gem_name|
    download_gem(gem_name)
    unpack_gem(gem_name)
end
