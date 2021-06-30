require 'rspec/core/rake_task'

def target_dir
    File.join(__dir__, "target", "debug")
end

def prepare_environment
    if Gem.win_platform?
        ENV["RUBY_DLL_PATH"] = target_dir
    else
        # Used by ffi gem
        ENV["LD_LIBRARY_PATH"] = target_dir
        ENV["DYLD_LIBRARY_PATH"] = target_dir
        ENV["DYLD_FALLBACK_LIBRARY_PATH"] = target_dir
    end
end

RSpec::Core::RakeTask.new(:test) do |t|
    t.rspec_opts = ["-I#{target_dir} -Ilib"]
end

task :cargo_build do
    system("cargo build", exception: true)
end

task :cargo_test do
    system("cargo test", exception: true)
end

task :compile do
    FileUtils.cp_r(File.join("extension", "."), target_dir)

    Dir.chdir(target_dir) do
        FileUtils.mkdir_p("include")
        FileUtils.mkdir_p("lib")

        FileUtils.cp("bloom.h", "include", verbose: true)
        
        if Gem.win_platform?
            FileUtils.cp(["bloom.d", "bloom.dll", "bloom.dll.exp", "bloom.dll.lib", "bloom.pdb"], "lib", verbose: true)
        else
            Dir.glob("libbloom.*").each do |filename|
                destination = File.join("lib", filename)
                FileUtils.cp_r(filename, destination, verbose: true)
            end
        end

        system("ruby extconf.rb --with-bloom-dir=#{Dir.pwd}", exception: true)
        system("make", exception: true)        
    end
end

task :console => [:compile] do
    prepare_environment

    exec("irb -I#{target_dir} -Ilib -rbloom_filter -rbloom_ffi -rbloom_ruby")
end

task :bench do
    prepare_environment

    file_path = File.join("lib", "bloom_bench.rb")

    exec("ruby -I#{target_dir} -Ilib -rbenchmark #{file_path}")
end

task :clean do
    system("cargo clean", exception: true)
end

task :default => [:cargo_build, :cargo_test, :compile, :test]
