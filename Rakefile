require 'rspec/core/rake_task'

RSpec::Core::RakeTask.new(:test) do |t|
    t.rspec_opts = ["-I#{__dir__}/target/debug"]
end

task :compile_cargo do
    system('cargo build', exception: true)
end

task :compile_c do
    target_dir = 'target/debug'

    FileUtils.cp_r('extension/.', target_dir)

    Dir.chdir(File.join(__dir__, target_dir)) do
        system('ruby extconf.rb', exception: true)
        system('make', exception: true)
    end
end

task :compile => [:compile_cargo, :compile_c]

task :console => [:compile] do
    require "#{__dir__}/target/debug/bloom_filter"
    require "irb"

    ARGV.clear()
    IRB.start()
end

task :default => [:compile, :test]
