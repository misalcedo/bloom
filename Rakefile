task :compile_cargo do
    system('cargo build', exception: true)
end

task :compile_c do
    target_dir = 'target/debug'

    FileUtils.cp('wrapper/extconf.rb', target_dir)
    FileUtils.cp('wrapper/bloom_filter.c', target_dir)

    Dir.chdir(File.join(__dir__, target_dir)) do
        system('ruby extconf.rb', exception: true)
        system('make', exception: true)
    end
end

task :compile => [:compile_cargo, :compile_c] do
end

task :console => [:compile] do
    require "#{__dir__}/target/debug/bloom_filter"
    require "irb"

    ARGV.clear()
    IRB.start()
end

task :default => :console
