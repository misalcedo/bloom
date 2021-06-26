require 'rspec/core/rake_task'

def target_dir
    File.join(__dir__, "target", "debug")
end

RSpec::Core::RakeTask.new(:test) do |t|
    t.rspec_opts = ["-I#{target_dir}"]
end

task :compile_cargo do
    system("cargo build", exception: true)
end

task :compile_c do
    FileUtils.cp_r(File.join("extension", "."), target_dir)

    Dir.chdir(target_dir) do
        FileUtils.mkdir_p("include")
        FileUtils.mkdir_p("lib")

        FileUtils.cp("bloom.h", "include", verbose: true)
        
        if Gem.win_platform?
            FileUtils.cp(["bloom.d", "bloom.dll", "bloom.dll.exp", "bloom.dll.lib"], "lib", verbose: true)
        else
            Dir.glob("libbloom.*").each do |filename|
                if File.symlink?(filename)
                    FileUtils.copy_entry(filename, File.join("lib", filename), verbose: true, remove_destination: true)
                else
                    FileUtils.cp(filename, "lib", verbose: true)
                end
            end
        end

        system("ruby extconf.rb --with-bloom-dir=#{Dir.pwd}", exception: true)
        system("make", exception: true)        
    end
end

task :compile => [:compile_cargo, :compile_c]

task :console => [:compile] do
    require File.join(target_dir, "bloom_filter")
    require "irb"

    ARGV.clear()
    IRB.start()
end

task :clean do
    system("cargo clean", exception: true)
end

task :default => [:compile, :test]
