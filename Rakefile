require 'rspec/core/rake_task'

def target_dir
    File.join(__dir__, "target", "debug")
end

RSpec::Core::RakeTask.new(:test) do |t|
    ENV["RUBY_DLL_PATH"] = target_dir if Gem.win_platform?
    t.rspec_opts = ["-I#{target_dir}"]
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
            system("gendef bloom.dll", exception: true)
            FileUtils.cp(["bloom.d", "bloom.def", "bloom.dll", "bloom.dll.exp", "bloom.dll.lib", "bloom.pdb"], "lib", verbose: true)
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
    ENV["RUBY_DLL_PATH"] = target_dir if Gem.win_platform?

    exec("irb -I#{target_dir} -rbloom_filter")
end

task :clean do
    system("cargo clean", exception: true)
end

task :default => [:cargo_build, :cargo_test, :compile, :test]
